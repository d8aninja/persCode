def add_binary(a,b):
    return bin( int( a + b) ).lstrip('0b')

if __name__ == "__main__":
    add_binary(1,1)