fn main() {
    let n: i32 = 10; // Example: 10th Fibonacci number
    let ans = fib(n);
    println!("Fibonacci({}) = {}", n, ans);
}

fn fib(num: i32) -> i32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

    let mut first = 0;
    let mut second = 1;

    for _ in 2..=num {
        let temp = second;
        second = second + first;
        first = temp;
    }
    second
}
