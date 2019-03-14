use std::collections::HashMap;

fn main() {
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);
    dbg!(&scores1); // ref bc need scores1 later

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect();
    dbg!(&scores2); // need scores2 later

    // values like i32 that are copy are copied in the hashmap
    // owned values like string are moved into the hashmap

    let field_name1 = String::from("Favorite Color");
    let field_value1 = String::from("Blue");
    let field_name2 = String::from("Favorite Number");
    let field_value2 = 99.to_string(); // must be same type as inferred by inital insertion

    let mut map1 = HashMap::new();
    // remember, references would need lifetimes
    map1.insert(field_name1, field_value1); // field_name1, field_value1 are invalid after this point
    map1.insert(field_name2, field_value2); // field_name2, field_value2 are invalid after this point
    dbg!(&map1); // ref bc need map1 later

    let field_name3 = String::from("Weight_LB");
    let field_value3 = 200;
    let field_name4 = String::from("Height_IN");
    let field_value4 = 73;

    let mut map2 = HashMap::new();
    // remember, references would need lifetimes
    map2.insert(field_name3, field_value3); // field_name3 ...
    map2.insert(field_name4, field_value4); // and field_name4 go out of scope here
    dbg!(map2); //map2 goes out of scope here
    println!("{} and {} are still available bc copy!", field_value3, field_value4);

    let get_name = String::from("Blue");
    let score = scores2.get(&get_name);
    // println!("{}", score); // can't, get returns generic Option(&V) aka concrete Some(&10)
    dbg!(score); // score = Some(10)

    for (key, value) in &scores2 {
        println!("{}: {}", key, value );
    }

    // demo hashmap overwrite
    map1.insert(String::from("Favorite Color"), String::from("Red")); // was "Blue";
    dbg!(map1); // now shows overwrite "Red"

    // demo hashmap entry check
    scores1.entry(String::from("Yellow")).or_insert(80);
    scores1.entry(String::from("Blue")).or_insert(80);
    scores1.entry(String::from("Red")).or_insert(80);
    println!("{:?}", scores1);

    // word counter
    let text = "hello world wonderful world hello";
    let mut word_map = HashMap::new();
    for word in text.split_whitespace(){
        // or_insert returns a mutable reference &mut V
        let count = word_map.entry(word).or_insert(0); // can't be ref, because lifetime ends on this iter: dangling reference
        *count += 1; // in order to change the value of a mutable reference we must dereference it first (?)
    }
    println!("{:?}", word_map);
}