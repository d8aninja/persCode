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
from collections import defaultdict
d = defaultdict()
for key, value in pairs:
    d[key].append(value)