// Define a struct 'User' that holds a reference to a string slice.
// The lifetime parameter <'a> tells Rust how long the reference is valid.
struct User<'a> {
    name: &'a str, // 'name' is a reference to a string slice with lifetime 'a
}

fn main() {
    // Create a String on the stack
    let name = String::from("Abir Dutta");
    // Create a User struct that borrows 'name'.
    let user = User {
        name: &name,
    };
    // Print the name stored in the User struct
    println!("{}", user.name);
}

 