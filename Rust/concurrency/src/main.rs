// Tasks
// 1. Spawn two threads:
// 2. One thread counts from 1 to 5.
// 3. Another thread prints letters a to e.
// 4. Use thread::sleep to simulate work.
// 5. Bonus: Use a channel to send both numbers and letters back to the main thread, and print them there.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
fn channel_communication() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["one", "two", "three"];
        for val in vals {
            println!("Pushing...");
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    println!("Main thread's out...");
    for received in rx {    // blocks until all tx references from all threads are released
        println!("Got: {}", received);
    }

    println!("The main thread is done doing stuff...");
}

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for n in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("{} thread started!", n);
            let mut num = counter.lock().unwrap();
            *num += 1;
            thread::sleep(Duration::from_millis(500));
        });
        println!("pushing thread {}", n);
        handles.push(handle);
    }
    println!("pushed all threads to handles...");

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
