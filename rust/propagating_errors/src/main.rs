use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    // force the compiler to give you the type by giving the wrong annotionation 
    // (enum `std::result::Result<std::fs::File, std::io::Error>`)
    // let f: u32 = File::open("hello.txt");

    let f: Result<_,_> = File::open("hello.txt");
    // dbg!(&f);
    
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file, but failed: {:?},", e),
            }
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    dbg!(&f);

    // more betterly:
    let g = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    dbg!(&g);

    // let h = File::open("this_file_doesnt_exist.txt").unwrap();
    // similar to unwrap, but lets you specify the panic message
    let h = File::open("this_file_doesnt_exist.txt").expect("Can't open file!");

}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_better() -> Result<String, io::Error> {
    // v1
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // v2
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;
    // Ok(s)
    
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
    
    // could technically write just this, but its not as clear
    // std::fs::read_to_string("hello.txt")
}