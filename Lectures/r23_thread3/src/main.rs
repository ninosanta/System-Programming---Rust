use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/** Sometimes a program has to throw some thread and then wait for them to reach a critical point
 *  before continuing.
 *  To wait a reasonable amount of time, no sleep is done because it would be impossible to previously
 *  know how many time it would require for them.
 *  A countdown latch data structure is used instead. */

mod cdl {
    use std::sync::{Condvar, Mutex};

    pub struct Cdl {  // countdown latch
        m: Mutex<usize>, // mutex to protect the countdown i.e., the shared state
        cv: Condvar,     // it is used when the wait is done while the mutex does not contain 0 yet
    }

    impl Cdl {
        pub fn new(count: usize) -> Self {
            Cdl {
                m: Mutex::new(count),
                cv: Condvar::new(),
            }
        }

        pub fn count_down(&self) {
            let mut s = self.m.lock().unwrap();  /* s is a SmartPointer that
                                                                   * represents the state of the
                                                                   * Countdown and it is dereferenceable */
            if *s == 0 {
                panic!("Count is 0 already");
            }
            *s -= 1;
            if *s == 0 {
                self.cv.notify_all();  // wake up all those that are waiting
            }
        }

        pub fn wait(&self) {
            let mut s = self.m.lock().unwrap();
            while *s > 0 {  // if not 0 yet, sleep
                s = self.cv.wait(s).unwrap();
            }
        }
    }
}

fn main() {
    let cdl = Arc::new(cdl::Cdl::new(2));  // we want to wait for 2 threads
    let cdl1 = Arc::clone(&cdl);
    let cdl2 = Arc::clone(&cdl);

    let t1 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        cdl1.count_down();  // initialized

        sleep(Duration::from_secs(5));
        println!("Thread1 done");
    });

    let t2 = thread::spawn(move || {
        sleep(Duration::from_secs(3));
        cdl2.count_down();  // initialized

        sleep(Duration::from_secs(6));
        println!("Thread2 done");
    });

    /* on the main thread */
    cdl.wait();
    println!("Done");
    t1.join().unwrap();
    t2.join().unwrap();
}
