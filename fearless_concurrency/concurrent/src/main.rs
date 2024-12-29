use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi nubmer {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // here main thread is locked to start before the handle thread is finished
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
    // here the main is locked and not finished before handle thread is finished
    // handle.join().unwrap();

    let v = vec![1, 2, 3];

    // Without the move Rust can’t tell how long the spawned thread will run,
    // so it doesn’t know if the reference to v will always be valid
    // closure tries to borrow v (because println only needs borrow and it's infered calling a closure),
    // but the owner is thread and the thread could be dropped immediatly
    // so we force the closure to take ownership of the value
    let handle_vec = thread::spawn(move || {
        println!("Here is the vector: {v:?}");
    });

    handle_vec.join().unwrap();

    // channels
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("one"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
            // println!("val is {val}"); // we can't borrow val after moving transfer ownership to channel
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
            // println!("val is {val}"); // we can't borrow val after moving transfer ownership to channel
        }
    });
    for recieved in rx {
        println!("Got: {recieved}");
    }

    // mutexes
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
    let num = m.lock().unwrap();
    println!("Underlaying value in mutex is: {}", num);

    // Sharing mutex between threads

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
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
    println!("Result: {}", *counter.lock().unwrap());
}
