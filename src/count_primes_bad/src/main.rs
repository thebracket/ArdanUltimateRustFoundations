fn is_prime(n: u32) -> bool {
    (2 ..= n/2).all(|i| n % i != 0 )
 }

fn main() {
    const MAX: u32 = 200_000;
    let mut counter = 0;
    let t1 = std::thread::spawn(|| {
       counter += (2 .. MAX/2).filter(|n| is_prime(*n)).count();
    });
    let t2 = std::thread::spawn(|| {
       counter += (MAX/2 .. MAX).filter(|n| is_prime(*n)).count();
    });
    t1.join();
    t2.join();
    println!("Found {counter} prime numbers in the range 2..{MAX}");
 }