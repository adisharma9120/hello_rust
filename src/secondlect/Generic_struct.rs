struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    let int_pair = Pair {
        first: 1,
        second: 2,
    };
    let str_pair = Pair {
        first: "hello",
        second: "world",
    };

    println!("Int pair: {} {}", int_pair.first, int_pair.second);
    println!("Str pair: {} {}", str_pair.first, str_pair.second);
}