p = (4,5)
x,y = p
print(x)
print(y)

data = ['ACME', 50, 91.1, (2012,12,21)]
name, shares, price, date = data
print(name)
print(shares)
print(date)
name, shares, price, (year, month, day) = data
print(year)
print(month)

# unpacking works with any iterable (strings, files, iterators, generators)
s = 'Hello'
a,b,c,d,e = s
print(a)
print(b)
print(c)

records = [
    ('foo', 1, 2),
    ('bar', 'hello'),
    ('foo', 3, 4),
]
def do_foo(x, y):
    print('foo', x, y)
    
def do_bar(s):
    print('bar', s)
    
for tag, *args in records:
    if tag == 'foo':
        do_foo(*args)
    elif tag == 'bar':
        do_bar(*args)
        
line = 'nobody:*:-2:2:Unpriv_user:/var/empty:/usr/bin/false'
uname, *fields, homedir, sh = line.split(':')

record = ('ACME', 50, 123.45, (12, 18, 2018))
name, *_, (*_, year) = record

# list proc; note recursion isn't a strong python feature
items = [1,2,7,8,12]
head, *tail = items
def sum(items):
    head, *tail = items
    return head + sum(tail) if tail else head
sum(items)