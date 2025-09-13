fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let iter = v1.iter();

    for n in iter {
        println!("{}", n * 2);
    }
}
