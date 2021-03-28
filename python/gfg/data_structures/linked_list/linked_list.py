#! /usr/bin/env python3

class Node():
    
    def __init__(self, data):
        self.data = data
        self.next = None

class LinkedList():

    def __init__(self):
        self.head = None

    def printList(self):
        temp = self.head
        while temp:
            print(temp.data)
            temp = temp.next
    # O(1); 4 steps
    def push(self, new_data):
        # step 1&2: initialize new node and load data
        new_node = Node(new_data)
        # step 3: make the new node's .next the old head
        new_node.next = self.head
        # step 4: make the new node the new head
        self.head = new_node
    # O(1); 5 steps
    def insert(self, prev_node, new_data):
        # step 1: check if previous node exists
        if prev_node is None:
            print("Previous node must be in the given linked list.")
            return
        # step 2&3: initalize new node and load data
        new_node = Node(new_data)
        # step 4: point new node's .next at previous node's .next
        new_node.next = prev_node.next
        # step 5: point prev_node's .next at new_node
        prev_node.next = new_node
    # O(n) unless you have a pointer to the final node, iwc O(1)
    def append(self, new_data):
        # steps 1-3: create, load, point
        new_node = Node(new_data)
        # step 4: if LinkedList instance is empty, new_node is the new .head
        if self.head is None:
            self.head = new_node
            return
        # step 5: else traverse to last node
        last = self.head
        while (last.next):
            last = last.next
        # step 6: change (true) last's .next node to new_node
        last.next = new_node

if __name__ == "__main__":
    import argparse
    from argparse import RawTextHelpFormatter

    parser = argparse.ArgumentParser(formatter_class=RawTextHelpFormatter, description="""\n
    LINKED LIST: Instantiate a linked list.

    DESCRIPTION:
    - Like an array, is a linear data structure.
    - Unlike an array, is stored in a non-contiguous location.
    - Non-contiguity is overcome by the use of pointers.
    - Pointers (for each element) are an extra memory requirement.
    WHY TO USE:
    - Dynamic size: arrays require a fixed upper-limit, known in advance, regardless of use.
    - Ease of insertion / deletion (dyn. arrays are O(n)).
    WHY NOT TO USE: 
    - Random access is not allowed. We have to access elements sequentially starting from the first node.
      So we cannot do binary search with linked lists efficiently with its default implementation.
    - Extra memory space for a pointer is required with each element of the list.
    - Not cache friendly. Since array elements are contiguous locations, there is locality of reference which is not
      there in case of linked lists.
    COMPLEXITIES (Wikipedia):
                              Linked list   Array   Dynamic array   Balanced tree
    Indexing                          Θ(n)   Θ(1)       Θ(1)             Θ(log n)
    Insert/delete at beginning        Θ(1)   N/A        Θ(n)             Θ(log n)
    Insert/delete at end              Θ(1)   N/A        Θ(1) amortized   Θ(log n)
    Insert/delete in middle     search time
                                    + Θ(1)   N/A        Θ(n)             Θ(log n)
    Wasted space (average)            Θ(n)    0         Θ(n)             Θ(n)
    """)
    args = parser.parse_args()

    ll_1 = LinkedList()
    ll_1.head = Node(1)

    second = Node(2)
    third = Node(3)

    ll_1.head.next = second
    second.next = third
    ll_1.printList()

    ll_1.push("real 1")
    ll_1.insert(ll_1.head.next, "inserted node")
    ll_1.append("the end")
    ll_1.printList()

    ll_2 = LinkedList()
    ll_2.append(6)
    ll_2.push(1)
    ll_2.printList()

    ll_1.insert(ll_1.head.next, ll_2) # this only inserts the pointer(...?)
    ll_1.printList()
    # real 1
    # 1
    # <__main__.LinkedList object at 0x7fd16c8a9b50>
    # inserted node
    # 2
    # 3
    # the end