fn main() {
    // Create a String containing a full name
    let name = String::from("Abir Dutta");
    
    // Call the function to get the first name
    let first_name = find_first_name(&name);
    println!("{}", first_name); // Print the first name
}
   
// This function returns a slice of the first name from the input string
fn find_first_name(name: &String) -> &str {
    let mut index = 0;
    // Loop through each character in the string
    for i in name.chars() {
        if i != ' ' {
            index = index + 1; // Move to the next character
        } else {
            // Return a slice from the start up to the space
            return &name[0..index];
        }
    }
    // If there is no space, return the whole string
    return &name[0..index];
}   