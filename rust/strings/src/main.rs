fn main() {

    let _s1 = String::new(); //  unused, just for init method

    let data = "This is some data we might store in a string; \
            but it could be byte code.\nThis UTF-8 encoded string has the Display trait.";

    println!("{}", &data); // ref to retain scope

    // need mut here if we want to push_str later
    // can bind the orig data obj because above was ref
    let mut s1 = data.to_string(); // has the Display trait
    s1.push_str(" More text!"); // needs to be mutable
    println!("{}", s1); // consumed/moved; s1 goes out of scope

    //alt
    let s2 = String::from(data); // same as to_string
    println!("{}", s2);

    // anything UTF-8 encoded
    let gretting1 = String::from("안녕하세요");
    let gretting2 = String::from("السلام عليكم");
    println!("{}.\n{}.", gretting1, gretting2);

    let mut s3 = String::from("foo"); // mut
    let s4 = "bar"; // not mut
    s3.push_str(s4); // str is copy...
    println!("s1 is {}. s2 is {}", s3, s4); // ... so we can still use it here

    let mut s5 = String::from("lo");
    s5.push('l'); // s5 must be mut; single quote
    println!("{}", s5);

    let hello = String::from("Hello, ");
    let world = String::from("world!");
    // the + operator uses the add method (sig: fn add(self, s: &str) -> String)
    // so we need to use a &ref when + concatenating strings...
    let hello_world = hello + &world; // ...hello is moved here, can non longer be used...
    println!("{} Hello, {}", hello_world, world); // ...but world can.

    let s6 = "tic";
    let s7 = "tac";
    let s8 = "toe";
    // format doesn't take ownership of any of its params; println does
    let s9 = format!("{}-{}-{}!", s6, s7, s8);
    println!("{}", s9);

    // English chars are 1B and therefore 1 length each
    let s10 = "Hello";
    let len1 = s10.len();
    println!("{}", len1);
    // but not all UTF-8 chars are 1B
    let len2 = String::from("Здравствуйте").len(); // 12?...
    println!("{}", len2); // ...24! 2 bytes per character!
 
    // ...so you can't index into strings like most languages 
    // let h = s10[0]; // this will fail, try slices to be more specific:
    let h = &s10[0..1];
    println!("{}", h);

    for c in "नमस्ते".chars() {
        println!("{}", c); // missing 4th and 6th unicode scalar values (USV)
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b); // 18 bytes for 6 Hindu USVs!
    }
}