def fib(n):
    i = 1
    x = [0,1]
    while len(x) < n:
        x.append(x[i] + x[i-1])
        i += 1
    return(x)

# alternatively
def fibon(n):
    a = b = 1
    result = []
    for i in range(n):
        result.append(a)
        a, b = b, a + b
    return result

# generator version, does not block as much memory
def fibGen(n):
    a = b = 1
    for i in range(n):
        yield a
        a, b = b, a + b
fibGen(100)


def fibRatio(s):
    # if called on fib(min(n,3)) == golden ratio 
    if len(s) < 3:
        raise ValueError("Need at least 2, non-zero elements to calculate Fibbonacci ratios.")
    else:
        i = 3
        l = [0,(s[2]/s[1])]
        while len(l) < len(s)-1:
            l.append(float(s[i])/s[i-1])
            i += 1
        return(l)

even_fibSequence = filter(lambda x: x % 2 == 0
fibRatio(even_fibSequence, fib(100))) # around 1 + golden ratio

def multiply(x):
    return (x*x)
def add(x):
    return (x+x)

from collections import OrderedDict
funcs = [multiply, add]
names = ['a', 'b', 'c']
vals = [1,2,3]
d = OrderedDict(zip(names,vals))
for i in d.values():
    value = list(map(lambda x: x(i), funcs))
    print(value)

some_list = ['a', 'b', 'c', 'b', 'd', 'm', 'n', 'n']
unqs = set([x for x in some_list])
dupelicates = set([x for x in some_list if some_list.count(x) > 1])

valid = set(['yellow', 'red', 'blue', 'green', 'black'])
input_set = set(['red', 'brown'])
# valid
print(input_set.intersection(valid))
# invalid
print(input_set.difference(valid))

