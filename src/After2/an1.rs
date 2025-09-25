/* fn main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    println!("Numbers: {:?}", numbers);

    if let Some(last) = numbers.pop() {
        println!("Popped: {}", last);
    }

    for n in &numbers {
        println!("Remaining: {}", n);
    }
}



fn main() {
    println!("hello world");
    let name = "Adiii";
    let age = 22;
    let weight = 43.4;

    println!("Name : {}", name);
    println!("age : {}", age);
    println!("weight :{}", weight);

    let sum = 4 + 10;
    println!("{}", sum);

    // Conditional example
    if weight > 70.0 {
        println!("You are above 70 kg.");
    } else {
        println!("You are 70 kg or less.");
    }
}

*/
// main.rs
fn main() {
    // Call a function
    greet("Aditya");

    // Array example
    let numbers = [10, 20, 30, 40, 50];
    println!("Numbers in the array: {:?}", numbers);

    // Sum of array elements
    let total: i32 = sum_array(&numbers);
    println!("Sum of numbers: {}", total);

    // Tuple example
    let person: (&str, u32, f32) = ("Aditya", 22, 72.5);
    println!(
        "Name: {}, Age: {}, Weight: {}",
        person.0, person.1, person.2
    );

    // Conditional check with tuple
    if person.2 > 70.0 {
        println!("You are above 70 kg.");
    } else {
        println!("You are 70 kg or less.");
    }
}

// Function to greet a person
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// Function to sum elements of an array
fn sum_array(arr: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in arr {
        sum += num;
    }
    sum
}
