def fibGen(n):
    a = b = 1
    for i in range(n):
        yield a
        a, b = b, a + b
even_fibSequence = filter(lambda x: x % 2 == 0, fibGen(33))
sum(even_fibSequence)
