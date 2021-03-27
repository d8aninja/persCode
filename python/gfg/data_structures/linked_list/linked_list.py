#! /usr/bin/env python3

def linked_list():
    """
    LINKED LIST:
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
    COMPLEXITIES (Source: Wikipedia):
                              Linked list   Array   Dynamic array   Balanced tree
    Indexing                          Θ(n)   Θ(1)       Θ(1)             Θ(log n)
    Insert/delete at beginning        Θ(1)   N/A        Θ(n)             Θ(log n)
    Insert/delete at end              Θ(1)   N/A        Θ(1) amortized   Θ(log n)
    Insert/delete in middle     search time 
                                    + Θ(1)   N/A        Θ(n)             Θ(log n)
    Wasted space (average)            Θ(n)    0         Θ(n)             Θ(n)
    """
    pass

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Instantiate a linked list.")
    parser.add_argument('--doc', dest="doc", required=False, type=int, default=1)

    args = parser.parse_args()

    if args.doc:
        print(linked_list.__doc__)
