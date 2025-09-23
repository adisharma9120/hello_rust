/* use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for n in numbers {
            println!("Number: {}", n);
        }
    });

    handle.join().unwrap();
}
 */


 // Multiple Threads

 use std::thread;

fn main() {
    let t1 = thread::spawn(|| {
        println!("Thread 1 is running");
    });

    let t2 = thread::spawn(|| {
        println!("Thread 2 is running");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}


