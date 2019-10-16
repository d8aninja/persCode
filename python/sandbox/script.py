class MyClass():
    """
    Basic docstring for test class case.
    """
    i = 123

    def speak():
        print "hello world!"

if __name__ == "__main__":
    MyClass.speak()
    print MyClass.i
    print MyClass.__doc__