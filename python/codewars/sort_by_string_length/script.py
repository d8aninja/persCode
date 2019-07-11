arr = ['beg', 'i', 'life', 'to']
li = ["Glasses", "Monocles", "Eyes", "Telescopes"]
ln = li
srt = dict()
for i, v in enumerate(li):
    print("%s: %s (%s)" % (i, v, len(v)))
    srt[v] = len(v)
    ln[i] = v
print(li)
srt = sorted(srt)
print(srt)