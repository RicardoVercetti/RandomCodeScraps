// Tasks
// 1. Spawn two threads:
// 2. One thread counts from 1 to 5.
// 3. Another thread prints letters a to e.
// 4. Use thread::sleep to simulate work.
// 5. Bonus: Use a channel to send both numbers and letters back to the main thread, and print them there.

use std::thread;
use std::time::Duration;

fn main() {
    let mut handles = vec![];
    
    for i in 1..=2 {
        let handle = thread::spawn(move || {
            for l in 1..=5 {
                println!("Counting {} from thread {}", l, i);
                thread::sleep(Duration::from_millis(500));
            }
        });
        handles.push(handle);
    }

    for i in 1..3 {
        println!("Hi from main thread: {}", i);
        thread::sleep(Duration::from_millis(500));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
