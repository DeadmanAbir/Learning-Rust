fn main() {
    // Call the function to find the first 'a' or 'A' in the string "bair"
    let index = find_alphabet(String::from("bair"));
    // You can also print the value directly if you want
    // println!("{}", index.to_string());

    // Check if the function found an 'a' or 'A'
    match index {
        Some(index) => println!("{}", index), // If found, print the position
        None => println!("Not found"),        // If not found, print this
    }
}

// This function looks for the first 'a' or 'A' in the given string
// If found, returns its position (index). If not, returns None.
fn find_alphabet(str: String) -> Option<u32> {
    for (index, char) in str.chars().enumerate() {
        if char == 'a' || char == 'A' {
            return Some(index as u32); // Found, return the position
        }
    }
    return None; // Not found, return None
}
