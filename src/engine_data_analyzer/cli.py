import argparse as ap
import logging

import numpy as np
import pandas
from rich.console import Console
from rich.logging import RichHandler

logger = logging.getLogger(__name__)


def time_as_tuple(seconds: float) -> tuple[int, int, float]:
    hours = int(seconds // 3600)
    minutes = int((seconds % 3600) // 60)
    secs = seconds % 60
    return hours, minutes, secs


def split_sessions(dataframe: pandas.DataFrame) -> list[pandas.DataFrame]:
    logger.info("Splitting sessions based on 'Session Time' column")
    # Identify session splits based on when the session time drops back to close to zero
    split_points_range = dataframe["Session Time"].shift(-1) < dataframe["Session Time"]
    split_points = [
        int(idx) for idx in split_points_range[split_points_range == True].index
    ]
    last_dataframe_idx = int(dataframe.index[-1])
    split_points[-1] = last_dataframe_idx

    sessions = []
    for i in range(len(split_points)):
        start = (split_points[i - 1] + 1) if i > 0 else 0
        end = split_points[i]
        session_df = dataframe.loc[start:end].copy()
        sessions.append(session_df)

    logger.info("Total sessions identified: %d", len(sessions))
    return sessions


def fill_missing_gps_time(dataframe: pandas.DataFrame) -> pandas.DataFrame:
    logger.info("Reconstructing missing GPS Date & Time values")
    # Fill missing GPS Date & Time values by interpolating based on Session Time
    if "GPS Date & Time" not in dataframe.columns:
        logger.warning(
            "No 'GPS Date & Time' column found, skipping GPS time reconstruction"
        )
        return dataframe

    gps_not_na = dataframe["GPS Date & Time"].notna()
    len_gps_not_na = gps_not_na.sum()
    gps_na_idx = dataframe["GPS Date & Time"].isna()
    len_gps_na = gps_na_idx.sum()

    if len_gps_not_na == 0:
        logger.debug("No GPS Date & Time found, skipping")
        return dataframe

    if len_gps_na == 0:
        logger.debug("No entries are NA, skipping")
        return dataframe

    logger.debug("Session has %d NA and %d not-NA values", len_gps_na, len_gps_not_na)

    session_time = np.concat(
        (
            dataframe["Session Time"][gps_not_na]
            .to_numpy()
            .reshape((len_gps_not_na, 1)),
            np.ones((len_gps_not_na, 1)),
        ),
        axis=1,
    )
    gps_time_type = dataframe["GPS Date & Time"].dtype
    gps_time = (
        dataframe["GPS Date & Time"][gps_not_na]
        .to_numpy()
        .astype(float)
        .reshape((len_gps_not_na, 1))
    )

    fit, _, _, _ = np.linalg.lstsq(session_time, gps_time)
    logger.debug("Found a fit with m=%d, c=%d", fit[0][0], fit[1][0])

    missing_gps_time_session_times = np.concat(
        (
            dataframe["Session Time"][gps_na_idx].to_numpy().reshape((len_gps_na, 1)),
            np.ones((len_gps_na, 1)),
        ),
        axis=1,
    )
    interpolated_gps_times = pandas.DataFrame(
        (missing_gps_time_session_times @ fit).astype(gps_time_type).flatten(),
        index=dataframe.index[gps_na_idx],
        columns=["GPS Date & Time"],
    )

    dataframe.update(interpolated_gps_times)

    return dataframe


def main() -> None:
    parser = ap.ArgumentParser()
    parser.add_argument("input_file", help="Input CSV file from engine monitor")
    parser.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="Increase verbosity level (can be used multiple times)",
    )
    args = parser.parse_args()
    input_file = args.input_file

    log_level = logging.WARNING  # Default log level
    if args.verbose == 1:
        log_level = logging.INFO
    elif args.verbose >= 2:
        log_level = logging.DEBUG

    logging.basicConfig(
        level=log_level,
        format="%(message)s",
        handlers=[RichHandler(console=Console(stderr=True))],
    )

    dataframe = pandas.read_csv(
        input_file,
        low_memory=False,
        parse_dates=True,
    ).copy()
    dataframe["GPS Date & Time"] = pandas.to_datetime(
        dataframe["GPS Date & Time"], errors="coerce"
    )
    logger.debug("Read dataframe with %d rows", len(dataframe))

    flight_sessions = split_sessions(dataframe)
    logger.debug("Found %d flight sessions", len(flight_sessions))

    # Parse each session
    for i, session_df in enumerate(flight_sessions):
        logger.info("Parsing session %d", i)
        logger.debug(
            "Session %d starts at index %d and ends at index %d",
            i,
            session_df.index[0],
            session_df.index[-1],
        )
        session_df = fill_missing_gps_time(session_df)
        gps_time_df = session_df["GPS Date & Time"].dropna()
        gps_time_start: pandas.Timestamp | None = None
        gps_end_time: pandas.Timestamp | None = None
        if gps_time_df.empty:
            print(f"Split {i + 1:3d}: (No GPS Time data available)")
        else:
            gps_time_start = gps_time_df.iloc[0]
            gps_end_time = gps_time_df.iloc[-1]
        duration = (
            session_df["Session Time"].iloc[-1] - session_df["Session Time"].iloc[0]
        )
        assert duration >= 0, "Duration should be non-negative"
        hours, minutes, secs = time_as_tuple(duration)
        hours_str = f"{hours:02d}h" if hours > 0 else ""
        minutes_str = f"{minutes:02d}m" if minutes > 0 else ""
        secs_str = f"{secs:.0f}s"
        print(
            f"Split {i + 1:3d}: (Duration: {hours_str:>3s} {minutes_str:>3s} {secs_str:>3s}, GPS Time: {gps_time_start} to {gps_end_time})"
        )


if __name__ == "__main__":
    main()
