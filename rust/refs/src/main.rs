fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing)
}

fn dangle() -> String {
//fn dangle() -> &String { this will dangle
    let s = String::from("hello");

    //&s this will dangle; return the String instead
    s //this moves ownership out; underlying value isn't deallocated
}
