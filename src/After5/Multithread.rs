/* use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        // thread::spawn starts a new OS thread.
        for i in 0..5 {
            println! {"Hi from spawn thread {}", i};
        }
    });
    for i in 0..50 {

        println! {"Hi from main thread {}", i};

    }
}
 */

// 2. Thread with Join (Wait for Completion)
use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Hello from a new thread!");
    });

    println!("Hello from the main thread!");
}
