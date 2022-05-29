use std::ops::Add;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

fn main() {
    /* we want to create a shared data structure that lives inside a mutex and is referenceable thanks to Arc */
    let state = Arc::new(Mutex::new(Vec::<String>::new()));
    let h: Vec<JoinHandle<()>> = (1..=2).map(|tid| {
        let state = state.clone();
        thread::spawn(move || {
            for i in 1..500 {
                state.lock().unwrap()
                    .push(i.to_string().add("-").add(tid.to_string().as_str()));  // the push is done only if the lock is owned
            }
        })
    }).collect();

    h.into_iter().for_each(|h| h.join().unwrap());
    println!("===============================================");
    state.lock().unwrap().iter().for_each(|s| println!("{}", s));
}
