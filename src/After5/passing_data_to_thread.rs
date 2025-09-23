use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for n in numbers {
            println!("Number: {}", n);
        }
    });

    handle.join().unwrap();
}
