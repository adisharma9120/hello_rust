fn main() {
    let a = 20;
    let b = 30;
    let sum = do_sum(a, b);
    println!("Sum is: {}", sum);
}

fn do_sum(a: i32, b: i32) -> i32 {
    a + b
}
