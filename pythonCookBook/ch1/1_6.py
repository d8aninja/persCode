from collections import defaultdict

d = defaultdict(list)
d['a'].append(1)
d['a'].append(2)
d['b'].append(4)
d

d = defaultdict(set)
d['a'].add(1)
d['a'].add(2)
d['b'].add(4)

# defaultdict creates entries for keys not accessed until later on, 
# if you don't want that use
d = {}
d.setdefault('a', []).append(1)
d.setdefault('a', []).append(2)
d.setdefault('b', []).append(4)

pairs = (
    ('jeff', 33),
    ('matt', 38),
    ('chris', 37),
    ('sean', 42)
)
d = {}
# see 1_15
for key, value in pairs:
    if key not in d:
        d[key] = []
    d[key].append(value)
    
# this is recommended but doesn't work?
d = defaultdict(list)
for key, value in pairs:
    d[key].append(value)