def my_decorator(func): # takes a reference
        def wrapper():
                print("Blah the First.")
                func() # calls that referenced function
                print("Blah the Last.")
        return wrapper # returns a reference!

@my_decorator
def middle_blah():
        print("Lord Middleblah, pleasure.")

if __name__ == "__main__":
    print(middle_blah)
    middle_blah()