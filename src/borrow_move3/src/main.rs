struct Data(usize);

impl Drop for Data {
    fn drop(&mut self) {
        println!("Data object {} is being destroyed", self.0);
    }
}

fn do_something(d: Data) -> Data {
    println!("Hello data #{}", d.0);
    d
}

fn main() {
    let data = Data(1);
    let data = do_something(data);
    do_something(data);
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Program ending");
}
