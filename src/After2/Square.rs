fn main() {
    let vec = Vec::new();

    let result = calculate_squares(vec);
    println!("Squares of numbers: {:?}", result);
}

fn calculate_squares(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for i in 0..10 {
        let square = i * i;
        println!("Number: {} -> Square: {}", i, square);
        new_vec.push(square);
    }

    return new_vec;
}
