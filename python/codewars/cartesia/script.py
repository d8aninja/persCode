class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def distance_to(self, pt):
        return math.sqrt( ((self.x - pt.x)**2) + ((self.y - pt.y)**2) )
        
class Line:
    def __init__(self, p1: Point, p2: Point):
        self.p1 = p1
        self.p2 = p2

    def length(self):
        return self.p1.distance_to(self.p2)

class Triangle:
    def __init__(self, p1: Point, p2: Point, p3: Point):
        self.p1 = p1
        self.p2 = p2
        self.p3 = p3
        self.area = 0.5 * self.base().length() * self.height().length()

    def base(self):
        return Line(self.p1, self.p2)
    
    def height(self):
        return Line(self.p2, self.p3)

def isValidWalk(walk):
    if len(walk) == 10:
        start = dict(x=0, y=0)
        path = start
        for step in walk:
            if step == 'n':
                path['y'] += 1
                print("after step %s path['y'] is %s" % (step, str(path['y'])))
            elif step == 'e':
                path['x'] += 1
                print("after step %s path['x'] is %s" % (step, str(path['x'])))
            elif step == 's':
                if path['y'] >= 0:
                    path['y'] -= 1 
                    print("after step %s path['y'] is %s" % (step, str(path['y'])))
                else: 
                    path['y'] -= 1
                    print("after step %s path['y'] is %s" % (step, str(path['y'])))
            elif step == 'w':
                if path['x'] >= 0:
                    path['x'] -= 1 
                    print("after step %s path['x'] is %s" % (step, str(path['x'])))
                else:
                    path['x'] -= 1
                    print("after step %s path['x'] is %s" % (step, str(path['x'])))
        if path['x'] == path['y'] == 0:
            print("\nFinal position of 'path' is %s,%s.\n" % (path['x'], path['y']))
            print("True! You're back in time!\n")
            return True
        else:
            print("\nFinal position of 'path' is %s,%s.\n" % (path['x'], path['y']))
            print("False! You didn't get back in time!\n")
            return False
    else:
        print("Not a 10-minute trip - False!\n")
        return False

if __name__ == "__main__":
    import  math

    p1 = Point(x = 1, y = 2) #doesn't actually type check, like Rust
    p2 = Point(x = 3, y = 2)
    p3 = Point(x = 3, y = 4)

    # side_1 = Line(p1, p2)
    # side_2 = Line(p2, p3)
    # side_3 = Line(p3, p1)
    # equal to these sides on this triangle; extract into Triangle()?

    triangle_1 = Triangle(p1, p2, p3)
    print("Triangle 1's area is %s\n" % (triangle_1.area))

    walk_false = ["s", "s", "w", "n", "n", "s", "e", "w", "n", "s"]
    walk_short = ["n", "e", "w", "s"]
    walk_true = ["n", "e", "w", "s", "n", "e", "w", "s", "n", "s"]
    walk_false_2 = ["e", "w", "s", "w", "w", "s", "n", "s", "e", "w"]

    isValidWalk(walk_false)
    isValidWalk(walk_short)
    isValidWalk(walk_true)
    isValidWalk(walk_false_2)

    # origin = dict(x = 0, y = 0)
    # start = origin # or any Point(x, y)
    # walk = ['n', 'n', 'e', 's', 'e', 's', 'w', 'w']

    # for step in walk:
    #     path = start
    #     if step == 'n':
    #         path['y'] += 1
    #         print("after step %s path['y'] is %s" % (step, str(path['y'])))
    #     elif step == 'e':
    #         path['x'] += 1
    #         print("after step %s path.x is %s" % (step, str(path['x'])))
    #     elif step == 's':
    #         path['y'] -= 1
    #         print("after step %s path['y'] is %s" % (step, str(path['y'])))
    #     elif step == 'w':
    #         path['x'] -= 1
    #         print("after step %s path.x is %s" % (step, str(path['x'])))
    # print("\nFinal position of 'path' is %s,%s.\n" % (path['x'], path['y']))

