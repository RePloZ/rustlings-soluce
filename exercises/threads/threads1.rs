// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM DONE

use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

impl JobStatus {
    fn new(completed: u32) -> JobStatus {
        JobStatus{jobs_completed: Mutex::new(completed)}
    }
}

fn main() {
//    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(JobStatus::new(0));
    let status_shared = Arc::clone(&status);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut jobs = status_shared.jobs_completed.lock().unwrap();
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            *jobs += 1;
            tx.send(*jobs).unwrap();
        }
    });
    
    let mut completed: u32 = 0;

    for received in rx {
        completed = received;
    }

    while completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
