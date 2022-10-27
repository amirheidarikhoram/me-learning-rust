use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    // create_threads_arc_mutes();

    create_threads_mpsc();
}

#[allow(dead_code)]
fn create_threads_arc_mutes() {
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

#[derive(Debug)]
#[allow(dead_code)]
struct Message {
    content: String,
    thread_id: u32,
}

#[allow(dead_code)]
fn create_threads_mpsc() {
    let mut handles = vec![];

    let (send, recv) = std::sync::mpsc::channel::<Message>();

    let mut cur = 0;
    static MAX_THREAD: u8 = 10;

    loop {
        let cloned_send = send.clone();
        let handle = thread::spawn(move || {
            let message = Message {
                content: String::from("Hello"),
                thread_id: cur,
            };

            cloned_send.send(message).unwrap();
        });

        handles.push(handle);

        cur += 1;
        if cur == MAX_THREAD.into() {
            break;
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    cur = 0;

    loop {
        let message = recv.recv().unwrap();
        println!("Message: {:?}", message);

        cur += 1;
        if cur == MAX_THREAD.into() {
            break;
        }
    }

}
