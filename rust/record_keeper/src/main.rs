#[derive(Debug)]
struct Validated {
    good: Option<String>,
    bad: Option<String>
}

fn main() {
    let mut v: Vec<Validated> = vec![];
    let x = Validated { good: Some("message".to_string()), bad: None };
    let y = Validated { good: Some("message".to_string()), bad: None };
    let z = Validated { good: Some("message".to_string()), bad: None };

//    let rec_x = Record::X(x);
//    let rec_y = Record::Y(y);
//    let rec_z = Record::Z(z);


    v.push(x);
    v.push(y);
    v.push(z);

    dbg!(v);


    let mut xs = vec![1i32, 2, 3];
    println!("\n\nInitial vector: {:?}", xs);

    let v = vec![0; 4];
    dbg!(v);
}
