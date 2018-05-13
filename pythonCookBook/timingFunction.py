import time

def timing_function(some_function):
    
    def wrapper(*args, **kwargs):
        t1 = time.time()
        some_function(*args, **kwargs)
        t2 = time.time()
        return "Time it took to run this function" + str((t2-t1))
    return wrapper

if __name__ == "main":
    timing_function()