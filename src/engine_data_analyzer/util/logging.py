"""Logging utilities"""

from contextlib import contextmanager
import time
import logging
from typing import Callable
import functools


def log_entry_and_exit(logger: logging.Logger):
    """Use the supplied logger to log both the entry and exit into a
    function. This decorator also logs the elapsed time in the
    function."""

    def decorator(f: Callable):
        @functools.wraps(f)
        def wrapper(*args, **kwargs):
            start_time = time.monotonic()
            logger.info(f"Entering {f.__name__}")
            ret = f(*args, **kwargs)
            end_time = time.monotonic()
            logger.info(f"Finished {f.__name__} in {end_time - start_time}")
            return ret

        return wrapper

    return decorator
