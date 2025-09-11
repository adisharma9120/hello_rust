use std::collections::HashMap;

fn main() {
    let mut cities = HashMap::new();

    cities.insert(String::from("Delhi"), 19000000);
    cities.insert(String::from("Mumbai"), 21000000);

    let delhi_population = cities.get("Delhi");
    println!("Population of Delhi: {:?}", delhi_population); // Some(19000000)
}
 