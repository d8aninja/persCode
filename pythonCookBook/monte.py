import random
from timingFunction import timing_function

def d(n):
    roll = random.randint(1,n)
    return roll

@timing_function
def rollD6(n = 6, k = 100):
    x = 0
    while x < k:
        result = d(n)
        print(result)
        x+=1


# CONSIDER SUBPROCESS DECORATORS!!!!! 