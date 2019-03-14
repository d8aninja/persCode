use std::f64::consts::PI as pi;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let a = pi;

    let b: f64 = pi;

    println!("The value of a double-precision, 64-bit float type '2.0' is {}", a);
    println!("The value of a single-point 32-bit ´float type '3.0' is {}", b);

    let check: bool = (3 == 4);

    let remainder: u32 = 53 % 10;

    println!("Check: {}", check);
    println!("Remainder: {}", remainder);

    let tupe: (i32, f64, u8) = (500, 6.4, 1); // pattern matching

    let (x, y, z) = &tupe; // 'deconstruction' w optional reference

    let (x2, y2, z2) = (tupe.0, tupe.1, tupe.2); // tuple indexing

    println!("Tuple: 'I'm holding {}, {}, and {}!'",
             tupe.0, tupe.1, tupe.2);
    println!("Deconstructed tuple: 'I'm holding {}, {}, and {}!'",
             x, y, z);
    println!("Matched, deconstructed tuple: 'I'm holding {}, {}, and {}!'",
             x2, y2, z2);

    let arry = [1,2,3,4,5]; // note type

    for i in &arry { // needs .iter or &ref, won't work on 'arry'
        println!("{}", i);
    }

    let names_array = ["Fred","Nick","Jeff"];

    let (fred, nick, jeff) = (names_array[0], names_array[1], names_array[2]);

    println!("The names in namesArray are {}, {}, and {}!",
        fred, nick, jeff);

}
