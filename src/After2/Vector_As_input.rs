fn main() {
    let vec = Vec::new();

    let result = takes_input(vec);
    println!("Final vector: {:?}", result);
}

fn takes_input(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();

    for i in 0..10 {
        if i % 2 == 0 {
            println!("Even no: {}", i);
            new_vec.push(i);
        } else {
            println!("Number is not even value: {}", i);
        }
    }

    return new_vec;
}
