fn main() {

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);
    let score = scores.get(&team_name).unwrap_or(&0);
    println!("{}", score);
    scores.entry(String::from("Red")).or_insert(80); // check if the key is inside hashmap. if yes, then ignore else insert key and value.

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);
    let maps = map.get(&team_name);// if the key is not inside it will return none instead of some
    println!("{:?}", maps);

    for (key, value) in &map {
        println!("{key}: {value}");
    }
    println!("{}", field_name);
    println!("{}", field_value);

    let text = "hello world wonderful world";

    let mut map1 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map1.entry(word).or_insert(0);// inserting 0 act as a mutable reference into count|| count = &mut i32
        println!("{word} : {count}");
        *count += 1;
        println!("{word} : {count}");
    }

    println!("{map1:?}");

}
