from collections import OrderedDict

# great for mapping that will later be serialized for encoded into a different format
d = OrderedDict()
d['foo'] = 1
d['bar'] = 2
d['spam'] = 3
d['grok'] = 4

for key in d:
    print(key, d[key])

# from above: eg precisely control the order of fields appearing in a JSON encoding
import json

json.dumps(d)

import pickle

pickle.dumps(d)

filename = open('picklefile', 'wb')
pickle.dump(d, filename)

filename = open('picklefile', 'rb')
unpickled = pickle.load(filename)