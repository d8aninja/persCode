pub fn mover() {
    let x = vec![1, 2, 3];
    let y = x;           // NOTE: x moved here (to fix, use .clone!)
    println!("{:?}", y); // OK 
    println!("{:?}", x); // ERROR: use of moved value x
}