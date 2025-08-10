
fn main() {
    // Create a vector with numbers 1 through 5
    let vec = Vec::from([1, 2, 3, 4, 5]);

    // Call the function to filter and double odd numbers, then print the result
    println!("{:?}", filter_and_double(vec));
}

// This function takes a vector, filters odd numbers, doubles them, and returns a new vector
fn filter_and_double(vec: Vec<i32>) -> Vec<i32> {
    vec.into_iter()
        .filter(|x| *x % 2 != 0) // Keep only odd numbers
        .map(|x| x * 2)         // Double each odd number
        .collect()              // Collect results into a new vector
}

