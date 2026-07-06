"""units.py

Defines units used in the program"""

from enum import Enum


class TemperatureUnit(Enum):
    UNKNOWN = 0
    CELSIUS = 1
    FARENHEIT = 2
