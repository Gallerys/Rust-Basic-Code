// This is a comment

// Declaring a variable
let my_variable = 10;

// Declaring a constant
const MY_CONSTANT: i32 = 5;

// Printing to the console
fn main() {
    println!("Hello, Rust!");

    // Conditional statement
    if my_variable > MY_CONSTANT {
        println!("my_variable is greater than MY_CONSTANT");
    } else if my_variable == MY_CONSTANT {
        println!("my_variable is equal to MY_CONSTANT");
    } else {
        println!("my_variable is smaller than MY_CONSTANT");
    }

    // Loop
    for i in 1..=5 {
        println!("Iteration {}", i);
    }

    // Vector (dynamic array)
    let my_vector = vec![1, 2, 3, 4, 5];

    // Loop through vector
    for number in &my_vector {
        println!("{}", number);
    }

    // Function
    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    let greeting = greet("Alice");
    println!("{}", greeting);
}
