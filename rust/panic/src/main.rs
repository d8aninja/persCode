fn main() {
    println!("Line one!");
    // panic!("Crash n' burn.");
    println!("Line two!");

    let v = vec![1, 2, 3];
    v[99]; // RUST_BACKTRACE=1 cargo run
}
