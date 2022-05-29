extern crate core;

use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

fn main() {
    let v: Vec<JoinHandle<i32>> = (1..=4).map(|tid| {
            thread::spawn(move || {  // "move" transfers to the thread the ownership
                (1..10).for_each(|i| {
                    println!("thread #{}, iteration #{}", tid, i);
                    sleep(Duration::from_secs(1));  // wait 1 second
                });
                if tid == 3 { panic!("thread #{} computation failed", tid); }
                println!("thread #{}, done!", tid);
                tid  // return value
            })
        }
    ).collect();

    println!("Main thread!");
    sleep(Duration::from_secs(4));
    println!("Waiting for secondary threads termination...");
    v
        .into_iter()  // one handle at a time
        .for_each(|h| {
            let res = h.join();  // join gives a Result because a thread may be stopped
                                            // before its actual termination
            match res {
                Ok(val) => println!("Result: {}", val),
                Err(e) => println!("Error {:?}", e)
            }
        });
    println!("Main thread has finished too");
}
