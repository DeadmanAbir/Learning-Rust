// Import the HashMap type from the standard library
use std::collections::HashMap;

fn main() {
    // Create a new, empty HashMap
    let mut map = HashMap::new();
    // Insert key-value pairs into the HashMap
    map.insert("key", "value");
    map.insert("key2", "value2");

    // Try to get the value for the key "key"
    let check = map.get("key");

    // Match on the result: Some if found, None if not
    match check {
        Some(value) => println!("{}", value), // Print the value if found
        None => println!("No Value found!!!!"), // Print a message if not found
    }
}

