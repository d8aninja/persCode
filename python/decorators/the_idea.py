def parent(num):
    def first_child():
        print("I'm the first child.")

    def second_child():
        print("I'm the second child")
    
    if num == 1: 
        return first_child # returns a reference
    else:
        return second_child # returns a reference

def my_decorator(func): # takes a reference
        def wrapper():
                print("Blah the First.")
                func() # calls that referenced function
                print("Blah the Last.")
        return wrapper # returns a reference!

def middle_blah():
        print("Lord Middleblah, pleasure.")

if __name__ == "__main__":
    first = parent(1)
    second = parent(2) 

    print(first)
    print(second)

    first()
    second()

    print("\n")
    middle_blah = my_decorator(middle_blah)
    print(middle_blah)
    middle_blah()


