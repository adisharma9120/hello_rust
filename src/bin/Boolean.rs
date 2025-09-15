fn main() {
    let ans = is_even(22);
    println!("{}", ans);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
