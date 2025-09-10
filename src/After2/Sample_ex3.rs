use std::collections::HashMap;

fn main() {
    let mut cars = HashMap::new();

    cars.insert(String::from("BMW"), 240);
    cars.insert(String::from("Audi"), 220);

    let bmw_speed = cars.get("BMW");
    println!("BMW top speed: {:?} km/h", bmw_speed); // Some(240)
}
