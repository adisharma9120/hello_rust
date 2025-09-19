fn main() {
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
