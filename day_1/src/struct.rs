// Define a struct to represent a student
struct Student {
    name: String,     // Student's name
    age: u32,        // Student's age
    learning: String // What the student is learning
}

// Implementation block for Student
impl Student {
    // Method to get the subject the student is learning
    fn subject(&self) -> String {
        return self.learning.clone();
    }
    // Static-like method that returns 21
    fn debug() -> i32 {
        return 21;
    }
}

fn main() {
    // Create a new Student instance
    let student = Student {
        name: String::from("John"),
        age: 21,
        learning: String::from("Rust"),
    };
    // Print what the student is learning
    println!("{}", student.subject());
    // Call the static-like debug function
    println!("{}", Student::debug());
}
