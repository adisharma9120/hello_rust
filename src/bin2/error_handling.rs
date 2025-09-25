fn main() {
    let numerator = 10;
    let denominator = 0;

    let result = if denominator != 0 {
        Some(numerator / denominator) // successful division
    } else {
        None // handle divide-by-zero
    };

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Error: Cannot divide by zero!"),
    }
}

