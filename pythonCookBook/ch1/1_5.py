# implementing a priority queue
import heapq


class priorityQueue:
    def __init__(self):
        self._queue = []
        self._index = 0

    def push(self, item, priority):
        heapq.heappush(self._queue, (-priority, self._index, item))
        self._index += 1

    def pop(self):
        return heapq.heappop(self._queue)[-1]


class Item:
    def __init__(self, name):
        self.name = name

    def __repr__(self):
        return 'Item({!r})'.format(self.name)


q = priorityQueue()
q.push(Item('foo'), 1)
q.push(Item('bar'), 5)
q.push(Item('spam'), 4)
q.push(Item('grok'), 1)
q.pop()
# Item('bar') bc priority 5 -> -5 and pop -> ...[-1]
q.pop()
# Item('spam')
q.pop()
# Item('foo') bc index incrementation -> fifoh
q.pop()
# Item('grok')

a = (1, Item('foo'))
b = (2, Item('bar'))
a < b
c = (1, Item('grok'))
a < c # no index

a = (1, 0, Item('foo'))
b = (2, 1, Item('bar'))
c = (1, 2, Item('grok'))
a < b
a < c
# for using queues to communicate between threads we need locking and signaling, 12_3