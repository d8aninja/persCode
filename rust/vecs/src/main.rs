#![allow(unused_variables)]
fn main() {
    // let v: Vec<T> = Vec::new(); // doesn't work (yet)
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];
    dbg!(v1);
    dbg!(v2);

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2]; // don't actually need &i32
    println!("The third element is {}", third);

    match v3.get(4) {
        Some(fifth) => println!("The fifth element is {}", fifth),
        None => println!("There is no fifth element."),
    }

    let mut v4 = Vec::new();

    v4.push(8);
    v4.push(9);
    v4.push(10);
    dbg!(&v4); // if not ref, v4 for-loop below will panic

    for i in &v3 {
        println!("{}", i);
    }

    for i in &mut v4 {
        *i *= 2;
    }

    dbg!(v4); // v4 is consumed by / moves into dbg, and goes out of scope after called.

    #[derive(Debug)]
    enum Counters {
        Int(i32), // ts
        Text(String), // name
        Float(f64), // value
    }

    let mut row = vec![
        Counters::Int(1552413607),
        Counters::Text(String::from("Bytes")),
        Counters::Float(26.626)
    ];

    dbg!(&row); // only consumes a reference (ptr copy)
    // row.push([1552413667, "Bytes", "26.626"])
    row.pop();
    dbg!(row);
}
