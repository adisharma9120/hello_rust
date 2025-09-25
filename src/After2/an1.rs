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


 */
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
