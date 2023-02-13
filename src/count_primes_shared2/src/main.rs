use std::thread::JoinHandle;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    const N_THREADS: u32 = 8;

    // Hold thread handles
    let mut threads: Vec<JoinHandle<Vec<u32>>> = Vec::with_capacity(N_THREADS as usize);

    // Generate all the numbers we want to check
    let group = MAX / N_THREADS;

    let now = std::time::Instant::now();

    for i in 0 .. N_THREADS {
        let counter = i;
        threads.push(std::thread::spawn(move || {
            let range = u32::max(2, counter*group) .. (i+1)*group;
            range.filter(|n| is_prime(*n)).collect()
        }));
    }

    let mut primes = Vec::new();
    for thread in threads {
        if let Ok(new_primes) = thread.join() {
            primes.extend(new_primes);
        } else {
            println!("Something went wrong");
        }
    }
    
    let duration = now.elapsed();
    println!("Found {} prime numbers in the range 2..{MAX}", primes.len());
    println!("Execution took {} seconds", duration.as_secs_f32());
 }