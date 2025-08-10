// Enum to represent different shapes
enum Shape {
    Rectangle(f64, f64), // Stores width and height
    Circle(f64),         // Stores radius
}

fn main() {
    // Make a rectangle shape with width 1.0 and height 2.0
    let rect: Shape = Shape::Rectangle(1.0, 2.0);

    // Make a circle shape with radius 1.0
    let circle = Shape::Circle(1.0);

    // Print the area of the rectangle
    println!("{}", calculate_area(rect));
}

// Function to find the area of a shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        // For rectangles: area = width * height
        Shape::Rectangle(width, height) => width * height,
        // For circles: area = pi * radius * radius (using 3.14 for pi)
        Shape::Circle(radius) => 3.14 * radius * radius,
    }
}