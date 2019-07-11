a = [121, 144, 19, 161, 19, 144, 19, 11]  
b = [121, 14641, 20736, 361, 25921, 361, 20736, 361]

def comp(array1, array2):
    try:
        return sorted([i ** 2 for i in array1]) == sorted(array2)
    except:
        return False

def comp(a1, a2):
    return None not in (a1, a2) and [i * i for i in sorted(a1)] == sorted(a2)