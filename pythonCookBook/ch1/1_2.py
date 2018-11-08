# requires python 3
import numpy as np

def drop_first_last(grades):
    first, *middle, last = grades
    return np.mean(middle)

this_terms_grades = (123, 231, 342, 234, 266)
drop_first_last(this_terms_grades)


def avg_comparison(sales_record):
    *trailing_qtrs, this_qtr = sales_record
    trailing_avg = sum(trailing_qtrs) / len(trailing_qtrs)
    print("This quarter's sales are " + 
          str(round((this_qtr/trailing_avg)*100, 2)) + 
          "% of the average of the last " + 
          str(len(trailing_qtrs)) + " quarters'.")

latest_five_quarters = [17.3, 23.2, 24.1, 27.3, 26.7]
avg_comparison(latest_five_quarters)