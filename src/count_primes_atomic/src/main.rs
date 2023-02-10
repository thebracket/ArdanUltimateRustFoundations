use std::sync::atomic::AtomicUsize;

fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let now = std::time::Instant::now();
    let t1 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (2 .. MAX/2).filter(|n| is_prime(*n)).count(), 
            std::sync::atomic::Ordering::Relaxed
        );
    });
    let t2 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (MAX/2 .. MAX).filter(|n| is_prime(*n)).count(), 
            std::sync::atomic::Ordering::Relaxed
        );
    });
    t1.join();
    t2.join();
    let duration = now.elapsed();
    println!("Found {} prime numbers in the range 2..{MAX}", COUNTER.load(std::sync::atomic::Ordering::Relaxed));
    println!("Execution took {} seconds", duration.as_secs_f32());
 }