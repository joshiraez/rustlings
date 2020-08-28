// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM NOT DONE

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_counter = Arc::clone(&status);
    thread::spawn(move || {
        for _ in 0..10 {
            let status_shared = Arc::clone(&status);

            thread::sleep(Duration::from_millis(250));

            let mut status_to_update = status_shared.lock().unwrap();
            status_to_update.jobs_completed += 1;
        }
    });
    let mut counter = status_counter.lock().unwrap().jobs_completed;
    while counter < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
        counter = status_counter.lock().unwrap().jobs_completed;
    }
}
