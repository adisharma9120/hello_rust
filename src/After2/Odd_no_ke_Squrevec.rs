fn main() {
    let vec = Vec::new();

    let result = takes_input(vec);
    println!("Final vector of odd squares: {:?}", result);
}

fn takes_input(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for i in 0..10 {
        if i % 2 != 0 {
            println!("Odd no: {} -> Square: {}", i, i * i);
            new_vec.push(i * i);
        } else {
            println!("Even number skipped: {}", i);
        }
    }

    return new_vec;
}
