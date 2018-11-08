from collections import deque

q = deque(maxlen=3)
q
q.append([1,2,3])
q
q.append(4)
q
q.append(5)
q.append(6)
q
q.pop()
q
q.popleft()
q
q.appendleft(4)
q