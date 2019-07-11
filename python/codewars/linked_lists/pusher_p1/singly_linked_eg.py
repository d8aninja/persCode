#https://www.codefellows.org/blog/implementing-a-singly-linked-list-in-python/

class Node(object):
    def __init__(self, data, next_node=None):
        self.data = data
        self.next_node = next_node

    def get_data(self):
        return self.data
    
    def get_next(self):
        return self.next_node
    
    def set_next(self, new_node):
        self.next_node = new_node

class LinkedList(object):
    def __init__(self, head=None):
        self.head = head # why not Node(head)?

    def insert(self, data):
        new_node = Node(data)
        new_node.set_next(self.head) # why not self.get_data()?
        self.head = new_node

    def size(self):
        current = self.head
        count = 0
        while current:
            count += 1
            current = current.get_next()
        return count

    def search(self, data):
        current = self.head
        found = False
        while current and found is False:
            if current == data:
                found = True
            else:
                current = current.get_next()
        if current is None:
            raise ValueError("Data not in list.")
        return current

    def delete(self, data):
        current = self.head
        previous = None
        found = False
        while current and found is False:
            if current.get_data() == data:
                found = True
            else:
                previous = current
                current = current.get_next()
        if current is None:
            raise ValueError("Data is not in list")
        if previous is None: # don't understand this block
            self.head = current.get_next()
        else:
            previous.set_next(current.get_next())


if __name__ == "__main__":
    # basic, single node
    # node_one = Node(1)
    # print(node_one)
    # print(node_one.get_data())

    ll = LinkedList('boop')
    ll.insert(1)
    print(ll)
    print(ll.head)
    # ll.head.get_data()
    # ll.head.get_next()

    # what about initializing a node that points to another?
    # shouldn't there be two nodes? yet... only one object...
    # node_two = Node(1, 2)
    # print(node_two.get_data())
    # print(node_two.get_next())