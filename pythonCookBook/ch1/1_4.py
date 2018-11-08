import heapq

numlist = [12,28,72,-23,49,25,-14,85,47,32]
print(heapq.nlargest(3, numlist))
print(heapq.nsmallest(3, numlist))

portfolio = [
    {'name':'IBM', 'shares':200, 'price':72.32},
    {'name':'AAPL', 'shares':100, 'price':300.10},
    {'name':'NFLX', 'shares':150, 'price':89.50},
    {'name':'NVDA', 'shares':250, 'price':18.24},
    {'name':'YHOO', 'shares':400, 'price':12.67},
    {'name':'FB', 'shares':350, 'price':63.14}
]
cheap = heapq.nsmallest(3, portfolio, key = lambda s: s['price'])
expensive = heapq.nlargest(3, portfolio, key = lambda s: s['price'])

# heap[0] is always smallest
# heap.heappop is the O(logN) method for finding largest/smallest
heap = list(numlist)
heapq.heapify(heap)
# type(num_heap)
heapq.heappop(heap)
heapq.heappop(heap)
heapq.heappop(heap)
heap # empty
numlist # full