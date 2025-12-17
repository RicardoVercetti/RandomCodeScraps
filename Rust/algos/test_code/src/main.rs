use std::time::{Instant, Duration};

fn main() {
    println!("Hello from the test code...");        // i5-10210U (8) @ 4.20 GHz [26_287_068, 26_955_637, 26_127_817, 26_586_694, 25_811_802, 26_270_351, 26_773_800, 26_777_096, 25_648_700, 25_715_188]

    let runs = 1_000_000;

    // let count = 0;
    let mut last_tick = Instant::now();
    println!("start time: {:?}", last_tick);
    let mut counts: Vec<i32> = Vec::new();
    let mut count = 0;

    for _i in 0..runs {
        for _j in 0..runs {
            count +=1;
            if last_tick.elapsed() >= Duration::from_secs(1) {
                // print!("\r(i,j) = ({}, {})", i, j);
                print!("\rcount: {}\n", count);
                println!("\rarr: {:?}", counts);
                counts.push(count.clone());
                count = 0;
                last_tick = Instant::now();
            }
        }
    }

}

