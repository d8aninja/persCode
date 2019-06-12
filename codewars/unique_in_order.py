str1 = 'AAAABBBCCDAABBB'
str2 = 'ABBCcAD'
l1 = [1,2,2,3,3]

def unique_in_order(itr):
    items = []
    prev = None
    for i in itr:
        if i != prev:
            items.append(i)
            prev = i
    return items

unique_in_order(str1)
unique_in_order(str2)
unique_in_order(l1)

# not as intuitive, but much shorter
# from itertools import groupby
# 
# def unique_in_order(itr):
#     return [k for (k, _) in groupby(itr)]