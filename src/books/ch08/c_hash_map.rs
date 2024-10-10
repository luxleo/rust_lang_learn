use std::collections::HashMap;

fn accessing_value_in_hash_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("BLUE"), 5);
    scores.insert(String::from("YELLOW"), 6);
    let team_name = String::from("BLUE");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score: {}", score);

    for (key,value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn hash_map_and_ownership() {
    let mut maps = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    maps.insert(field_name, field_value);

    // field_name and field_value pass ownership to maps
    // println!("{field_value} {field_name}"); occur error

    // to solve above problem we need lifetimes
}

fn update_hash_map() {
    // Method 1 : overriding value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // Method 2 : update only when key is absend
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Yellow")).or_insert(50);

    println!("{:?}", scores);

    // Method 3: updating a value based on existing value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        // count is reference to map's value , count == &mut map's V
        *count += 1;
    };
    println!("{:?}", map);
}