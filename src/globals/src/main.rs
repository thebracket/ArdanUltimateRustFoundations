use std::sync::Mutex;

use once_cell::sync::Lazy;

struct MyType(usize);

impl MyType {
    fn new(n: usize) -> Self {        
        Self(n)
    }
}

impl Drop for MyType {
    fn drop(&mut self) {
        println!("Drop");
    }
}

static SHARED: Lazy<Mutex<MyType>> = Lazy::new(|| Mutex::new(MyType::new(5)));

fn main() {
    println!("{}", SHARED.lock().unwrap().0);
}
