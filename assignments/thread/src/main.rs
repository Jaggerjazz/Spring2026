use std::sync::{Arc, Mutex};
use std::thread; // Keep this

fn main() {
    let total = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let cnt = total.clone();
        // Changed std::thread::spawn to just thread::spawn
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                *cnt.lock().unwrap() += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *total.lock().unwrap());
}