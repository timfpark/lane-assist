use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;

fn main() -> std::io::Result<()> {

    let loop_duration_target = Duration::from_millis(100);
    let mut rng = rand::thread_rng();

    // const MAX_ITERATIONS: i32 = 10000;
    const MAX_STRING_COPIES: i32 = 100000;

    let mut strings;

    loop {
        let start = SystemTime::now();
        let start_since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

        /*
        let iterations = rng.gen_range(0..MAX_ITERATIONS);

        // burn a random amount of CPU
        let mut total: u64= 0;
        for _ in 1..iterations {
            total += rng.gen_range(0..50);
        }

        println!("total is {}", total);
        */

        // reserve a random amount of memory (also burning random amounts of CPU in the process)
        let string_copies = rng.gen_range(0..MAX_STRING_COPIES);
        strings = Vec::new();
        for _ in 1..string_copies {
            strings.push(String::from("This is a string that we are adding to the vector to consume memory."));
        }

        let finish = SystemTime::now();
        let finish_since_the_epoch = finish.duration_since(UNIX_EPOCH).expect("Time went backwards");

        let consumed_time = finish_since_the_epoch - start_since_the_epoch;
        if consumed_time < loop_duration_target {
            let sleep_time = loop_duration_target - consumed_time;

            println!("sleeping for {}ms", sleep_time.as_millis());

            thread::sleep(sleep_time);
        }
    }
}
