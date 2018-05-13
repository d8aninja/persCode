from collections import defaultdict, OrderedDict

d = {
    'a': [1,2,3],
    'b': [4,5]
}

e = {
    'a': {1,2,3},
    'b': {4,5}
}

pairs = (
    ('jeff', 33),
    ('matt', 38),
    ('chris', 37),
    ('sean', 42)
)

laterPairs = (
    ('jeff', 34),
    ('matt', 39),
    ('chris', 38),
    ('sean', 43)
)

# consider
d = {}
for key, value in pairs:
    if key not in d:
        d[key] = []
    d[key].append(value)
for key, value in laterPairs:
    if key not in d:
        d[key] = []
    d[key].append(value)
    
# cleaner without needing the explicit initialization
d = defaultdict()
for key, value in pairs:
    d[key].append(value)
    
# 1.7: ordered dicts also ensure correct insertion order / mapping (think JSON):
d = OrderedDict()
d['foo'] = 1
d['bar'] = 2
d['spam'] = 3
d['grok'] = 4
for k in d:
    print(k, d[k])
d['foo'] = 5
for k in d:
    print(k, d[k])

## 1.8: also useful is inverting keys and values with zip for calcs
stocks = {
    'NFLX': 326.46,
    'AAPL': 188.59,
    'ATVI': 71.70,
    'NVDA': 254.53,
}

min_price = min(zip(stocks.values(), stocks.keys()))
max_price = max(zip(stocks.values(), stocks.keys()))

# remember, iterators are consumed! actually this doesn't work - or rather, it does! why?
prices_and_names = zip(stocks.values(), stocks.keys())
print(min(prices_and_names))
print(max(prices_and_names))

# compares only keys
min(stocks)
max(stocks)
# doesn't give names
min(stocks.values())
max(stocks.values())
# gets only names associated with min or max values
min(stocks, key = lambda k: stocks[k])
max(stocks, key = lambda k: stocks[k])
# for the value you'd need an extra lookup step:
stocks[min(stocks, key = lambda k: stocks[k])]

## 1.9: finding commonalities between dicts
a = {
    'x': 1,
    'y': 2,
    'z': 3
}
b = {
    'w': 10,
    'x': 11,
    'y': 2
}
# consider set ops
a.keys() & b.keys() # common
a.keys() - b.keys() # difference

# make a new dict with non-common keys removed
diffSet = {key:a[key] for key in a.keys() - b.keys()}

