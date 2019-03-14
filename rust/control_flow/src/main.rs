fn main() {
    let x: u8 = 3;

    if x < 5 {
        println!("Conditional was true.")
    } else {
        println!("Conditional was false.")
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num = if condition {
        5
    } else {
        6 // both arms must be same type because expression returns value to 'num'
    };

    println!("The value of 'num' is: {}", num);


}
