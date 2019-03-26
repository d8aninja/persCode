
enum Foo {
    Bar, 
    Baz,
    Qux(u32),
}

fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {},
    }

    // all i32
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    } 
    
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number, let's go with a letter!")
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters{
        println!("Didn't match a number, let's go with a letter!")
    } else {
        println!("I don't like letters - let's go with an emoticon :D", )
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar!");
    }

    if let Foo::Bar = b {
        println!("b is foobar!");
    } else {
        println!("b is not foobar!");
    }

    // Similar to Some(...)
    if let Foo::Qux(value) = c {
        println!("c is fooqux and {}!", value);
    }

    
}
