use std::collections::HashMap;

fn main() {
    let mut movies = HashMap::new();

    movies.insert(String::from("Inception"), 9.0);
    movies.insert(String::from("Interstellar"), 8.5);

    let inception_rating = movies.get("Inception");
    println!("Inception rating: {:?}", inception_rating); // Some(9.0)
}
