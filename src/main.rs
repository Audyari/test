use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // Membuat Arc (Atomic Reference Counting) untuk berbagi Mutex antar thread
    let counter = Arc::new(Mutex::new(0));

    // Membuat sebuah vector untuk menyimpan handles dari thread
    let mut handles = vec![];

    for _ in 0..10 {
        // Membuat Arc clone untuk setiap thread
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Mengunci mutex dan mengakses data
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}