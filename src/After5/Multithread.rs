use std::thread;

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
