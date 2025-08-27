// Tasks
// 1. Spawn two threads:
// 2. One thread counts from 1 to 5.
// 3. Another thread prints letters a to e.
// 4. Use thread::sleep to simulate work.
// 5. Bonus: Use a channel to send both numbers and letters back to the main thread, and print them there.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;
#[allow(unused_imports)]
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
    let mut handles = vec![];

    let handle1 = thread::spawn(|| {
        let nums = (1..=5).collect::<Vec<i32>>();
        for num in nums {
            println!("Num : {}", num);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handles.push(handle1);

    let handle2 = thread::spawn(|| {
        let alphs = ["a", "b", "c", "d", "e"];
        for alph in alphs {
            println!("alph : {}", alph);
            thread::sleep(Duration::from_millis(500));
        }
    });

    handles.push(handle2);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finished all...");
}
