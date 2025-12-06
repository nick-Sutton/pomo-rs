use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let time_limit = 30;

    // User entered a command to set the time -> Create a thread for the timer
    let handle = thread::spawn(move || {
        let mut curr_count = time_limit;
        let start_time = Instant::now();

        // When user enters the start command we start the timer

        while curr_count > 0 {
            let elapsed_time = start_time.elapsed();
            println!("Time remaining: {} ({}s)", curr_count, elapsed_time.as_secs_f64());

            // Pause the thread for 1 second
            thread::sleep(Duration::from_secs(1));
            curr_count -= 1;

            // Listen for stop command
        }
    });

    // Join the spawned thread
    handle.join().unwrap();

}
