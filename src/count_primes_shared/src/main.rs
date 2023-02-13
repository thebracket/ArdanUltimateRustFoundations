use std::sync::Mutex;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    const N_THREADS: u32 = 8;

    static PRIMES: Mutex<Vec<u32>> = Mutex::new(Vec::new());

    // Hold thread handles
    let mut threads = Vec::with_capacity(N_THREADS as usize);

    // Generate all the numbers we want to check
    let group = MAX / N_THREADS;

    let now = std::time::Instant::now();

    for i in 0 .. N_THREADS {
        let counter = i;
        threads.push(std::thread::spawn(move || {
            let range = u32::max(2, counter*group) .. (i+1)*group;
            let my_primes: Vec<u32> = range.filter(|n| is_prime(*n)).collect();
            PRIMES.lock().unwrap().extend(my_primes);
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }
    
    let duration = now.elapsed();
    println!("Found {} prime numbers in the range 2..{MAX}", PRIMES.lock().unwrap().len());
    println!("Execution took {} seconds", duration.as_secs_f32());
 }