use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    let handle = thread::spawn(move || {
        loop {
            let n = rx.recv().unwrap();
            match n {
                1 => println!("Hi from worker thread"),
                _ => break,
            }
        }
        println!("Thread closing cleanly");
    });

    for _ in 0..10 {
        tx.send(1).unwrap();
    }
    tx.send(0).unwrap();

    handle.join().unwrap();
}
