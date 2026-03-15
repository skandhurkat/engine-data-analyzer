import argparse as ap
import pandas


def time_as_tuple(seconds: float) -> tuple[int, int, float]:
    hours = int(seconds // 3600)
    minutes = int((seconds % 3600) // 60)
    secs = seconds % 60
    return hours, minutes, secs


def main() -> None:
    parser = ap.ArgumentParser()
    parser.add_argument("input_file", help="Input CSV file from engine monitor")
    args = parser.parse_args()
    input_file = args.input_file

    dataframe = pandas.read_csv(
        input_file,
        low_memory=False,
        parse_dates=True,
    ).copy()
    dataframe["GPS Date & Time"] = pandas.to_datetime(
        dataframe["GPS Date & Time"], errors="coerce"
    )

    # Identify session splits based on when the session time drops back to close to zero
    dataframe["split_points"] = (
        dataframe["Session Time"].shift(-1) < dataframe["Session Time"]
    )
    split_points = dataframe[dataframe["split_points"] == True].index
    print(f"Found {len(split_points)} sessions.")

    # Parse each session
    for i in range(len(split_points)):
        start = (split_points[i - 1] + 1) if i > 0 else 0
        end = split_points[i]
        gps_time_df = dataframe.loc[start:end, "GPS Date & Time"].dropna()
        gps_time_start: pandas.Timestamp | None = None
        gps_end_time: pandas.Timestamp | None = None
        if gps_time_df.empty:
            print(
                f"Split {i + 1:3d}: {start:6d} to {end:6d} (No GPS Time data available)"
            )
        else:
            gps_time_start = gps_time_df.iloc[0]
            gps_end_time = gps_time_df.iloc[-1]
        duration = (
            dataframe["Session Time"].iloc[end] - dataframe["Session Time"].iloc[start]
        )
        assert duration >= 0, "Duration should be non-negative"
        hours, minutes, secs = time_as_tuple(duration)
        hours_str = f"{hours:02d}h" if hours > 0 else ""
        minutes_str = f"{minutes:02d}m" if minutes > 0 else ""
        secs_str = f"{secs:.0f}s"
        print(
            f"Split {i + 1:3d}: {start:6d} to {end:6d} (Duration: {hours_str:>3s} {minutes_str:>3s} {secs_str:>3s}, GPS Time: {gps_time_start} to {gps_end_time})"
        )


if __name__ == "__main__":
    main()
