"""loader.py

Utilities for loading engine monitor CSV data and for cleaning up the
data to a consistent schema.
"""

import pandas as pd
import pathlib
import logging
import re

from engine_data_analyzer.util.logging import log_entry_and_exit
import engine_data_analyzer.core.units as units

logger = logging.getLogger(__name__)


@log_entry_and_exit(logger)
def read_file(file_path: str | pathlib.Path) -> pd.DataFrame:
    """Read a file at file_path and return a Pandas DataFrame object"""
    dataframe = pandas.read_csv(input_file, low_memory=False, parse_dates=True)


_cht_cols_regex = re.compile(r"^(?:CHT|cht)\s+(\d+)(\s+\(deg (C|F)\))?$")
_egt_cols_regex = re.compile(r"^(?:EGT|egt)\s+(\d+)(\s+\(deg (C|F)\))?$")


def __get_cyl_col_unit(col: str, regex: re.Pattern) -> units.TemperatureUnit:
    """Use the supplied regex to extract the units used for recording
    the given cylinder temperature. The supplied regex must capture the
    temperature as "deg C" or "deg F" in the second capture group."""
    m = regex.match(col)
    if m is None:
        raise ValueError(f"Cannot parse supplied column {col}")
    match_groups = m.groups()
    if len(match_groups) < 2 or match_groups[1] is None:
        return units.TemperatureUnit.UNKNOWN

    if "deg C" in match_groups[1]:
        return units.TemperatureUnit.CELSIUS
    elif "deg F" in match_groups[1]:
        return units.TemperatureUnit.FARENHEIT
    else:
        return units.TemperatureUnit.UNKNOWN


def __get_cyl_col_num(col: str, regex: re.Pattern) -> int | None:
    """Use the supplied regex to extract the cylinder number. The
    supplied regex must capture the cylinder number as the first
    capture group."""
    m = regex.match(col)
    if m is None:
        raise ValueError(f"Cannot parse supplied column {col}")
    match_groups = m.groups()

    if len(match_groups) < 1:
        logger.error(f"Error extracting cylinder number from {col}")
        return None

    try:
        return int(match_groups[0])
    except:
        logger.error(f"Error extracting cylinder number from {col}")
        return None


def _get_cht_col_unit(col: str) -> units.TemperatureUnit:
    """Extract the temperature unit used for logging CHT columns."""
    return __get_cyl_col_unit(col, _cht_cols_regex)


def _get_cht_col_cyl_num(col: str) -> int | None:
    """Extract the cylinder number from the CHT column."""
    return __get_cyl_col_num(col, _cht_cols_regex)


def _get_egt_col_unit(col: str) -> units.TemperatureUnit:
    """Extract the temperature unit used for logging EGT columns."""
    return __get_cyl_col_unit(col, _egt_cols_regex)


def _get_egt_col_cyl_num(col: str) -> int | None:
    """Extract the cylinder number from the EGT column."""
    return __get_cyl_col_num(col, _egt_cols_regex)
