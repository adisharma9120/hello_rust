fn main() {
    // Simple function call
    greet("Aditya");

    // Function with return value
    let sum = add(10, 20);
    println!("Sum is: {}", sum);

    // Function returning multiple values using tuple
    let (min, max) = find_min_max(5, 15);
    println!("Min: {}, Max: {}", min, max);
}

// Function without return (just prints)
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function with return value (-> i32 means returns integer)
fn add(a: i32, b: i32) -> i32 {
    a + b // last expression is the return value (no semicolon)
}

// Function returning tuple
fn find_min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
