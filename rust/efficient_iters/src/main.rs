fn main() {
    increment_str_bytes("Howdy!");
    increment_str_bytes("Every day I'm iterating!");

    let scores = vec!( ("Jeff".to_string(), 5) );
    let name =  pluck_first_name(scores);
    if name == ["Jeff"] {print!{"{}", "\nTrue!"}};
}

fn increment_str_bytes(s: &str) {
    let str = s;
    let string = String::from(str);
    let bytes = string.into_bytes();

    print!("\nOld: {:#?}", bytes);

    let bytes_into = bytes
        .into_iter()
        .map(| a | a + 1);

    for i in bytes_into { print!("\nNew: {}", i) }
}

fn pluck_first_name(v: Vec<(String, usize)>) -> Vec<String> {
    v.into_iter()
        .map(|(name, _score)| name)
        .collect()
}