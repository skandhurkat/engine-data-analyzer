import argparse as ap
import logging

import numpy as np
import pandas
from rich.console import Console
from rich.logging import RichHandler
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages
import matplotlib.dates as mdate

logger = logging.getLogger(__name__)


def time_as_tuple(seconds: float) -> tuple[int, int, float]:
    hours = int(seconds // 3600)
    minutes = int((seconds % 3600) // 60)
    secs = seconds % 60
    return hours, minutes, secs


def _split_dataframe_at_series(dataframe: pandas.DataFrame, series: pandas.Series):
    previous_index = dataframe.index[0]

    for split_point in dataframe[series].index:
        yield dataframe.loc[previous_index:split_point]
        previous_index = split_point + 1

    try:
        yield dataframe.loc[previous_index:]
    except UnboundLocalError:
        pass


def split_sessions(dataframe: pandas.DataFrame) -> list[pandas.DataFrame]:
    logger.info("Splitting sessions based on 'Session Time' column")

    # Identify session splits based on when the session time drops back to close to zero
    split_points = dataframe["Session Time"].shift(-1) < dataframe["Session Time"]

    sessions = [df for df in _split_dataframe_at_series(dataframe, split_points)]

    logger.info("Total sessions identified: %d", len(sessions))
    return sessions


def inject_nan_in_gaps(dataframe: pandas.DataFrame) -> pandas.DataFrame:
    split_positions = (
        dataframe["Session Time"].shift(-1) - dataframe["Session Time"] > 5
    )

    nan_df = pandas.DataFrame.from_dict({col: [np.nan] for col in dataframe.columns})

    fragments: list[pandas.DataFrame] = []
    for fragment in _split_dataframe_at_series(dataframe, split_positions):
        if len(fragments) != 0:
            filler_sess_time = (
                fragments[-1]["Session Time"].iloc[-1]
                + fragment["Session Time"].iloc[0]
            ) / 2
            nan_df.loc[0, "Session Time"] = filler_sess_time
            fragments.append(nan_df.copy())
        fragments.append(fragment)

    return pandas.concat(fragments)


def fill_missing_gps_time(dataframe: pandas.DataFrame) -> pandas.DataFrame:
    dataframe["GPS Date & Time"] = pandas.to_datetime(
        dataframe["GPS Date & Time"], errors="coerce"
    )
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

    logger.debug("Session has %d NA and %d not-NA values", len_gps_na, len_gps_not_na)

    if len_gps_not_na == 0 or len_gps_na == 0:
        logger.debug("Cannot interpolate, skipping")
        return dataframe

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
    m = fit[0][0]
    c = fit[1][0]
    logger.debug("Found a fit with m=%d, c=%d", m, c)
    if m < 990000 or m > 1010000:
        logger.error(
            "Computed GPS time to Session time slope m=%d out of range, interpolation will be incorrect",
            m,
        )
    if c < 0:
        logger.error(
            "Computed intercept c=%d less than zero, check GPS date and time", c
        )

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
    )["GPS Date & Time"].dt.round("s")

    dataframe.update(interpolated_gps_times)

    return dataframe


def is_engine_on(dataframe: pandas.DataFrame) -> bool:
    rpm_cols = [c for c in dataframe.columns if c.lower().startswith("rpm")]
    max_rpm = dataframe[rpm_cols].max(axis=1).max(axis=0)
    return max_rpm > 5.0


def get_low_oil_temp_high_rpm(
    dataframe: pandas.DataFrame,
) -> pandas.DataFrame:
    rpm_cols = [c for c in dataframe.columns if c.lower().startswith("rpm")]
    oil_temp_cols = [
        c for c in dataframe.columns if c.lower().startswith("oil temperature")
    ]
    max_rpm = dataframe[rpm_cols].max(axis=1)
    min_oil_temp = dataframe[oil_temp_cols].min(axis=1)

    anomaly = (max_rpm > 1500) & (min_oil_temp < 40)
    dataframe["Low Oil Temp & High RPM"] = anomaly

    return dataframe


def get_high_chts(dataframe: pandas.DataFrame) -> pandas.DataFrame:
    cht_cols = [c for c in dataframe.columns if c.lower().startswith("cht")]
    anomaly = dataframe[cht_cols].max(axis=1) > 200
    dataframe["High CHTs"] = anomaly
    return dataframe


def get_low_rpm_high_airspeed(dataframe: pandas.DataFrame) -> pandas.DataFrame:
    airspeed_cols = [
        c for c in dataframe.columns if c.lower().startswith("indicated airspeed")
    ]
    assert len(airspeed_cols) == 1
    rpm_cols = [c for c in dataframe.columns if c.lower().startswith("rpm")]
    max_airspeed = dataframe[airspeed_cols].max(axis=1)
    min_rpm = dataframe[rpm_cols].min(axis=1)
    anomaly = (max_airspeed > 120) & (min_rpm < 2250)
    dataframe["Low RPM & High Airspeed"] = anomaly
    return dataframe


def main() -> None:
    parser = ap.ArgumentParser()
    parser.add_argument("input_file", help="Input CSV file from engine monitor")
    parser.add_argument("output_file", help="Output PDF file")
    parser.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="Increase verbosity level (can be used multiple times)",
    )
    args = parser.parse_args()
    input_file = args.input_file
    output_file = args.output_file

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

    rpm_cols = [c for c in dataframe.columns if c.lower().startswith("rpm")]
    oil_temp_cols = [
        c for c in dataframe.columns if c.lower().startswith("oil temperature")
    ]
    cht_cols = [c for c in dataframe.columns if c.lower().startswith("cht")]
    airspeed_cols = [
        c for c in dataframe.columns if c.lower().startswith("indicated airspeed")
    ]

    with PdfPages(output_file) as pdf:
        # Parse each session
        for i, session_df in enumerate(flight_sessions):
            logger.info("Parsing session %d", i)
            logger.debug(
                "Session %d starts at index %d and ends at index %d",
                i,
                session_df.index[0],
                session_df.index[-1],
            )

            session_df = inject_nan_in_gaps(session_df)
            session_df = fill_missing_gps_time(session_df)

            gps_time_start = session_df["GPS Date & Time"].iloc[0]
            gps_time_end = session_df["GPS Date & Time"].iloc[-1]

            session_time_start = session_df["Session Time"].iloc[0]
            session_time_end = session_df["Session Time"].iloc[-1]
            duration = session_time_end - session_time_start
            if duration < 0:
                logger.error(
                    "Duration for session from %d to %d was negative, got %d. Session time at start is %d, at end is %d",
                    session_df.index[0],
                    session_df.index[-1],
                    duration,
                    session_time_start,
                    session_time_end,
                )

            assert duration >= 0, "Duration should be non-negative"
            hours, minutes, secs = time_as_tuple(duration)
            hours_str = f"{hours:02d}h" if hours > 0 else ""
            minutes_str = f"{minutes:02d}m" if minutes > 0 else ""
            secs_str = f"{secs:.0f}s"
            engine_on = is_engine_on(session_df)
            if not engine_on:
                logger.debug("Engine not turned on this session, skipping")
                continue

            print(
                f"Split {i + 1:3d}: (Duration: {hours_str:>3s} {minutes_str:>3s} {secs_str:>3s}, GPS Time: {gps_time_start} to {gps_time_end})"
            )

            session_df = get_low_oil_temp_high_rpm(session_df)
            if session_df["Low Oil Temp & High RPM"].any():
                print("Low Oil Temp & High RPM detected")

            session_df = get_high_chts(session_df)
            if session_df["High CHTs"].any():
                print("High CHTs detected")

            session_df = get_low_rpm_high_airspeed(session_df)
            if session_df["Low RPM & High Airspeed"].any():
                print("Low RPM & High Airspeed detected")

            # Plot the following in two plots
            # Plot 1: RPM and airspeed
            # Plot 2: CHTs and oil temperature
            fig, ax = plt.subplots(4, figsize=(8, 10.5))

            ax[0].plot(
                session_df["GPS Date & Time"],
                session_df[airspeed_cols],
                label=airspeed_cols,
            )
            ax[1].plot(
                session_df["GPS Date & Time"], session_df[rpm_cols], label=rpm_cols
            )
            ax[2].plot(
                session_df["GPS Date & Time"], session_df[cht_cols], label=cht_cols
            )
            ax[3].plot(
                session_df["GPS Date & Time"],
                session_df[oil_temp_cols],
                label=oil_temp_cols,
            )

            for i, a in enumerate(ax):
                a.legend()
                if i == 3:
                    # a.tick_params("x", rotation=45)
                    a.xaxis.set_major_formatter(
                        mdate.ConciseDateFormatter(a.xaxis.get_major_locator())
                    )
                else:
                    a.tick_params(
                        "x", which="both", bottom="off", top="off", labelbottom="off"
                    )

            fig.autofmt_xdate()

            pdf.savefig(bbox_inches="tight")

            plt.close(fig)


if __name__ == "__main__":
    main()
