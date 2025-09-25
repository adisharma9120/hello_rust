fn main() {
    // Array (same type values, fixed size)
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    println!("First number: {}", numbers[0]);
    println!("Third number: {}", numbers[2]);

    // Looping through array
    for num in numbers.iter() {
        println!("Array element: {}", num);
    }

    // Tuple (different types allowed)
    let person: (&str, i32, f64) = ("Aditya", 21, 72.5);
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Weight: {}", person.2);
}
