use std::collections::HashMap;

// Hash Maps
fn hash_maps_example() {
    // Define a hash map
    let mut scores = HashMap::new();

    // Insert key-value pairs into the hash map
    scores.insert(String::from("Alice"), 100);
    scores.insert(String::from("Bob"), 90);

    // Access values by key
    if let Some(score) = scores.get("Alice") {
        println!("Alice's score: {}", score);
    }
}

fn main() {
    hash_maps_example();
}
