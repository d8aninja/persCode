fn main() {

    let array = [1,2,3,4,5];

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{}", counter);
        if counter == array.len() {
            break counter * 2
        }
    };

    if result == 10 {
        print!("Okay!\n");
    };

    let mut t_minus = 10;

    while t_minus != 0 {
        println!("{}!", t_minus);
        t_minus -= 1;
    }; print!("Liftoff!\n");

    let mut index = 0;

    while index < 5 {
        println!("The value is {}", array[index]);
        index += 1;
    }; // this is error prone (ie if array changes).  better is:

    for element in array.iter() {
        println!("Iterated element: {}", element)
    }

    for i in (1..11).rev() {
        println!("{}!", i)
    }; println!("Liftoff!");
}
