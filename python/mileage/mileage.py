import argparse
from datetime import datetime, timedelta
import logging
import os
import re
import sys
from typing import Tuple

def dt_delta() -> timedelta:
    lease_date = datetime(2020, 9, 10)
    now = datetime.now()
    delta = now - lease_date
    return delta 

def mileage_calcs(odo: int = None) -> Tuple:
    daily_allowed_mileage = 36000 // (365 * 3)  # contract mileage // (DOY * contract # years)
    track = dt_delta() * daily_allowed_mileage
    index = 100*odo//track
    return ((odo, track), index)

if __name__ == "__main__":

    parser = argparse.ArgumentParser(description='Calculate and track lease mileage.')
    parser.add_argument('--odo', dest='current_odo', required=True)
    parser.add_argument('--year', dest='veh_year', required=False, default="2020")
    parser.add_argument('--make', dest='veh_make', required=False, default="Toyota")
    parser.add_argument('--model', dest='veh_model', required=False, default="Highlander")
    # parser.add_argument('--x', dest='x', action='store_true', default=False)
    args = parser.parse_args()

    run_name = args.veh_year + " " + args.veh_make + " " + args.veh_model
    print(run_name) if re.sub(r'\s+', '', run_name) != "" else print ("'Run Name' cannot be blank.")

    logger = logging.getLogger(run_name)
    logger.setLevel('INFO')
    
    try:
        d = dt_delta()
        mi = mileage_calcs(odo=args.current_odo)
        print(mi[1])
        print(mi[2])
        print(f"The lease is {d} days old; the {run_name} has ...")
    except TypeError as te:
        print(f"Error: {te}.\n")
        logger.warning(f"TypeError!\nMileage Var: {mi}.\nDelta Var: {d}.")
    except ValueError as ve:
        print(f"Could not convert data to an integer. Error: {ve}")
    except:
        print(f"Unexpected error: {sys.exc_info()[0]}")
        raise