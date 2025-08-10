fn main() {
    // Create a new, empty vector
    let mut vec = Vec::new();
    // Add some numbers to the vector
    vec.push(3);
    vec.push(5);
    vec.push(6);
    // Call the function to print even numbers
    filter_even(&mut vec);
    // Print the whole vector after filtering
    println!("{:?}", vec);
}

// This function prints even numbers from the given vector
fn filter_even(vec: &mut Vec<i32>) {
    for i in vec {
        if *i % 2 == 0 {
            println!("{}", i); // Print the even number
        }
    }
}