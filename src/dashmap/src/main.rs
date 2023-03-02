use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{thread, time::Duration};

static MAP: Lazy<DashMap<usize, usize>> = Lazy::new(DashMap::new);

fn main() {
    let mut threads = Vec::new();

    // Adder Threads
    for i in 0..10 {
        threads.push(thread::spawn(move || {
            for _ in 0..100 {
                if let Some(mut count) = MAP.get_mut(&i) {
                    *count += 1;
                } else {
                    MAP.insert(i, 1);
                }
                std::thread::sleep(Duration::from_secs_f32(0.1));
            }
        }));
    }

    // Reader Threads
    for i in 0..10 {
        threads.push(thread::spawn(move || {
            for _ in 0..20 {
                if let Some(count) = MAP.get(&i) {
                    println!("Count of {i}: {}", *count);
                    std::thread::sleep(Duration::from_secs_f32(0.5));
                }
            }
        }));
    }

    for t in threads {
        let _ = t.join();
    }
}
