use std::collections::HashMap;

fn main() {
    // create empty hash map and add to it
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // create by iterating over a vector of tuples
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = // specify what .collect() should return
        teams.into_iter() // converts teams to iterator
            .zip( // makes a list of tuples
                  initial_scores.into_iter() // converts initial_scores to iterator
            )
            .collect(); // makes the hash map from the tuples

    // Hash maps take ownership of types w/o the Copy trait (e.g. String)
    let field_name = "Favourite colour".to_string();
    let field_value = "Blue".to_string();

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // both variables are burnt now

    // get hash map values
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 50);

    // .get() returns Option<&v>
    let team_name = "Blue".to_string();
    let score = {
        match scores.get(&team_name) {
            Some(value) => *value,
            _ => 0,
        }
    };
    println!("{}", score);

    // iterate values
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // add new value and overwrite old
    scores.insert("Blue".to_string(), 25);
    println!("{:?}", scores);

    // add new value only if not already there using .entry()
    let mut scores = HashMap::new();
    scores.insert("Blue".to_string(), 10);
    scores.entry("Yellow".to_string()).or_insert(50);
    scores.entry("Blue".to_string()).or_insert(50);
    println!("{:?}", scores);
    println!("{:?}", scores.entry("Blue".to_string()));
    println!("{:?}", scores.entry("Red".to_string()));


    // update value based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // count = (&mut value)
        *count += 1; // dereference count with *
    } // all &mul values out of scope
    println!("{:?}", map);
}