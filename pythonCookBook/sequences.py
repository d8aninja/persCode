# for hashable types
def dedupeHash(items):
    seen = set()
    for item in items:
        if item not in seen:
            yield item # generator = general purpose!
            seen.add(item)

l = [1,2,3,3,4,3,2,2,2,1,1,1,1]
dGen = dedupeHash(l)
list(dGen) #dGen gets used up!

# for unhashable type (like dicts/maps)
def dedupeUnhash(items, key=None):
    seen = set()
    for item in items:
        # key: specify a function that converts sequence items
        # into a hashable type
        val = item if key is None else key(item)
        if val not in seen:
            yield item
            seen.add(val)

a = [ # all unhasable dicts...
    {'x': 1, 'y': 2},
    {'x': 1, 'y': 3},
    {'x': 1, 'y': 2}, # dupe
    {'x': 2, 'y': 4}
]
# dedupe on unique pairs of keys
list(dedupeUnhash(a, key = lambda d: (d['x'], d['y'])))
# dedupe on unique key x
list(dedupeUnhash(a, key = lambda d: (d['x'])))