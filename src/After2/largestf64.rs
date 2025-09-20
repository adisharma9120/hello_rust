fn main() {
    let bigger_float = largest_f64(3.14, 2.72);
    let biggger_bool = largest_bool(true, false);

    println!("bigger_float :{}", bigger_float);
    println!("biggger_bool : {}", biggger_bool);
}

fn largest_f64(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
fn largest_bool(a: bool, b: bool) -> bool {
    if a > b { a } else { b }
}
