// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 21 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..20 {
        let status_shared = status.clone();
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(1000));
            {
                status_shared.lock().unwrap().jobs_completed += 1;
            }
            println!("Job completed");
        });
        handles.push(handle);
    };

    let mut jobs_completed = 0;
    while jobs_completed < 20 {
        jobs_completed = status.lock().unwrap().jobs_completed;
        println!("waiting... {}", jobs_completed);
        thread::sleep(Duration::from_millis(500));
    }

    println!("Completed... {} jobs", status.lock().unwrap().jobs_completed);
    for handle in handles {
        handle.join().unwrap();
    }
}

// `Arc` is an Atomic Reference Counted pointer that allows safe, shared access
// to **immutable** data. But we want to *change* the number of `jobs_completed`
// so we'll need to also use another type that will only allow one thread to
// mutate the data at a time. Take a look at this section of the book:
// https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct
// and keep reading if you'd like more hints :)
//
//
// Do you now have an `Arc` `Mutex` `JobStatus` at the beginning of main? Like:
// `let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));`
// Similar to the code in the example in the book that happens after the text
// that says "We can use Arc<T> to fix this.". If not, give that a try! If you
// do and would like more hints, keep reading!!
//
//
// Make sure neither of your threads are holding onto the lock of the mutex
// while they are sleeping, since this will prevent the other thread from
// being allowed to get the lock. Locks are automatically released when
// they go out of scope.
//
// Ok, so, real talk, this was actually tricky for *me* to do too. And
// I could see a lot of different problems you might run into, so at this
// point I'm not sure which one you've hit :)
// Please open an issue if you're still running into a problem that
// these hints are not helping you with, or if you've looked at the sample
// answers and don't understand why they work and yours doesn't.
//
// If you've learned from the sample solutions, I encourage you to come
// back to this exercise and try it again in a few days to reinforce
// what you've learned :)
