use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    //basic_threading();
    //move_data_to_thread();
    //channels();
    //shared_state();
    deadlock();
}

pub fn deadlock() {
    let lock_1 = Arc::new(Mutex::new(0));
    let lock_2 = Arc::new(Mutex::new(0));

    let lock_1_copy = Arc::clone(&lock_1);
    let lock_2_copy = Arc::clone(&lock_2);

    let thread_1 = thread::spawn(move || {
        let num = lock_1.lock().unwrap();
        println!("{}", num);
        thread::sleep(Duration::from_secs(2));
        let num = lock_2.lock().unwrap();
        println!("{}", num);
    });
    let thread_2 = thread::spawn(move || {
        let num = lock_2_copy.lock().unwrap();
        println!("{}", num);
        thread::sleep(Duration::from_secs(2));
        let num = lock_1_copy.lock().unwrap();
        println!("{}", num);
    });

    thread_1.join().unwrap();
    thread_2.join().unwrap();
}

// shared state/memory concurrency
pub fn shared_state() {
    // cant use Rc because its unsafe across threads. Must use Arc
    //     let counter = Rc::new(Mutex::new(0));

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 1..=10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {:?}", counter);
}

// msg passing concurrency (like go channels)
// start up multi threads, multi producer, single receiver
pub fn channels() {
    let (tx, rx) = mpsc::channel();

    for i in 1..10 {
        let tx_clone = mpsc::Sender::clone(&tx);
        thread::spawn(move || loop {
            tx_clone
                .send(format!("hi from thread {}", i))
                .expect("Couldn't send!");
            thread::sleep(Duration::from_secs(1));
        });
    }

    for received in rx {
        println!("Got {}", received)
    }
}

// movie data from main thread to thread with move in closure
pub fn move_data_to_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

// basic threading example with wait on main thread
pub fn basic_threading() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // wait for thread to finish
    handle.join().unwrap();
}
