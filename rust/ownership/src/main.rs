fn main() {
    // s is not valid here - its not declared
    {
        let s = "Hello"; // s is a valid, fixed string-literal from this point forward
        // does 's' live in the stack or on the heap?
        println!("{}", s) // this works because s is in scope
    } // Rust calls `drop`, s ceases to be in scope, and is therefore invalid from this point on
    // println!("{}", s) // s "not found in this scope" - invalid; resulting compile-time error

    let mut s = String::from("Hello"); // 'new' s is now mutable; allocated on the heap

    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let _y = x; // y is unused, tag it w '_' so the compiler doesn't squeal
    // “bind the value 5 to x; then make a copy of the value in x and bind it to y.”
    // bc integers are simple, and a known, fixed size (eg i32), they are both pushed onto the stack
    // note this means they "are Copy types" and get shallow copy for free if you call them
    // `clone` wouldn't do anything different than copy here since x is on the stack

    let s1 = String::from("Hello");
    let s2 = s1;

    // println!("{}, world!", s1); // won't work, borrowing moved/invalidated reference
    println!("{}, world!", s2);

    let s3 = String::from("Check, check");
    let s4 = s3.clone(); // clone = deep copy: heap is copied

    // println!("{}, mic check!", s3); // won't work, borrowing moved/invalidated reference
    println!("{}, mic check!", s4);

    let s5 = String::from("Howdy!");  // s5 comes into scope

    takes_ownership(s5);            // s5's value moves into the function...
                                                // ... and so is no longer valid here

    let x2 = 5;                            // x2 comes into scope

    makes_copy(x2);              // x2 would move into the function,
                                               // but i32 is Copy, so it’s okay to still
    println!("After makes_copy: {}", x2);      // use x2 afterward

    let s6 = gives_ownership();          // gives_ownership moves its return
                                                // value into s6
    println!("{}", s6);

    let s7 = String::from("Give and take.");  // s7 comes into scope

    let s8 = takes_and_gives_back(s7);   // s7 is moved into
                                                          // takes_and_gives_back, which also
                                                          // moves its return value into s8
    println!("{}", s8);

    let s9 = String::from("Last one, with a length ... in a tuple!!");

    let (s10, len) = calculate_length(s9);

    println!("The length of '{}' is {}.", s10, len);

} // Here, x2 goes out of scope, then s. But because s5's value was moved, nothing
// special happens.  s8 goes out of scope and is dropped. s7 goes out of scope but was
// moved, so nothing happens. s6 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("I own this one: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// heap (value) and stack (pointer) memory are both freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("We made a copy! Here: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {            // gives_ownership will move its
                                            // return value into the function
                                            // that calls it

    let some_string = String::from("I'm s6!"); // some_string comes into scope

    some_string                             // some_string is returned and
                                            // moves out to the calling
                                            // function. don't use ;
}

// takes_and_gives_back will take a String and return one.
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}