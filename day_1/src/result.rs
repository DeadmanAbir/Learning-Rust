// Import the function to read file contents as a string
use std::fs::read_to_string;

fn main() {
    // Try to read the file using a function that returns Result
    let ans = read_file(String::from("file.txt"));

    // Try to read the file using a function that returns Option
    let option_ans = read_file_option(String::from("file.txt"));

    // Handle the Result: Ok means file was read, Err means there was a problem
    match ans {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }

    // Handle the Option: Some means file was read, None means not found or error
    match option_ans {
        Some(content) => println!("File content: {}", content),
        None => println!("File not found"),
    }
}

// This function tries to read a file and returns a Result
// Result is used when you want to know what went wrong
fn read_file(path: String) -> Result<String, String> {
    let result = read_to_string(path);

    // If reading was successful, return Ok with the content
    // If there was an error, return Err with the error message
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}

// This function tries to read a file and returns an Option
// Option is used when you only care if it worked or not
fn read_file_option(path: String) -> Option<String> {
    let option_result = read_to_string(path);

    // If reading was successful, return Some with the content
    // If there was an error, return None
    match option_result {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}