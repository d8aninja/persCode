import probables
from sizer import getsize

from probables import (BloomFilter)

blm = BloomFilter(est_elements=25000000, false_positive_rate=0.05)
line_ = 0
with open('war_and_peace.txt', 'r') as fp:
    for line in fp:
        getsizeof(line)
        for word in line.split():
            blm.add(word.lower())  # add each word to the bloom filter
        line_+=1
        print("\nline",str(line_),"\nblm size:",blm.export_size(),"\nCFPR:",blm.current_false_positive_rate())

