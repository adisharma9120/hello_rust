fn main() {
    let fruits = vec!["apple", "banana", "cherry"];
    println!("Fruits: {:?}", fruits);

    for (i, fruit) in fruits.iter().enumerate() {
        println!("Index {} => {}", i, fruit);
    }
}
