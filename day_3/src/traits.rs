// Define a trait (like an interface) with a method that must be implemented
pub trait StudentTrait {
    fn get_details(&self) -> String;
}

// Define a struct to represent a student
struct Student {
    name: String,
    subject: String,
}

// Implement the trait for the Student struct
impl StudentTrait for Student {
    fn get_details(&self) -> String {
        // Return a formatted string with the student's details
        return format!("{} is learning {}", self.name, self.subject);
    }
}

fn main() {
    // Create a new Student instance
    let student = Student {
        name: String::from("Abir Dutta"),
        subject: String::from("Rust"),
    };
    // Call the trait method on the student
    println!("{}", student.get_details());
}
   

