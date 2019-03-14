fn main() {
    println!("Hello, world!");

    another_function(5);

    let x = 6;

    println!("{}", {let x = 3;
        x + 1 // note the lack of semicolon => return value => block is an expression, not statement
    }); // if you add a ; it will not return a value, it becomes a statement
    println!("{}", x);

    let greetings = ["Hello", "Hola", "Bonjour", "Ciao", "こんにちは", "안녕하세요",
        "Cześć", "Olá", "Здравствуйте", "Chào bạn", "您好", "Hallo", "Hej", "Ahoj", "سلام"];
    for (num, greeting) in greetings.iter().enumerate() {
        print!("{}. {} World!\n", num + 1, greeting);
    }
}

fn another_function(x: i32) {
    println!("'another_function' prints {}!", x);
    println!("'ten' prints {}!", ten());
    let y = eight();
    println!("The value returned by 'eight' is {}...", y)
}

fn ten() -> i32 { // constrain the return value
    5*2
}

fn eight() -> u8 {
    8
}