import argparse
from datetime import datetime, timedelta
import logging
import os
import sys
from typing import Tuple

def dt_delta() -> timedelta:
    lease_date = datetime(2020, 9, 10)
    now = datetime.now()
    delta = now - lease_date
    return delta 

def mileage_calcs(current_odo: int = None) -> Tuple:
    daily_allowed_mileage = 36000 // (365 * 3)  # contract mileage // (DOY * contract # years)
    track = dt_delta() * daily_allowed_mileage
    index = 100*current_odo//track
    return ((current_odo, track), index)

if __name__ == "__main__":

    parser = argparse.ArgumentParser(description='Calculate and track lease mileage.')
    parser.add_argument('--odo', dest='current_odo', required=True)
    parser.add_argument('--name', dest='run_name', required=False)
    # parser.add_argument('--x', dest='x', action='store_true', default=False)
    args = parser.parse_args()
    logger = logging.getLogger(run_name if run_name is not None else "2020 Highlander")
