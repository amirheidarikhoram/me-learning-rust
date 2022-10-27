use std::{sync::{Mutex, Arc}, thread};

fn main() {
    let a = Arc::new(Mutex::new(0));

    let cloned_a = a.clone();

    let handle = thread::spawn(move || {
        let mut value = cloned_a.lock().unwrap();

        *value += 1;

        println!("Hello from a thread! {}", value);
    });
    handle.join().unwrap();

    let value = a.lock().unwrap();

    println!("Hello, world! {}", value);
}
