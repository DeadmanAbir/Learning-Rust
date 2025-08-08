fn main() {
    // Print the length (number of characters) of the string "21"
    println!("{}", star_length(String::from("21")));
}

// This function counts how many characters are in the given string
fn star_length(str: String) -> usize {
    return str.chars().count();
}
