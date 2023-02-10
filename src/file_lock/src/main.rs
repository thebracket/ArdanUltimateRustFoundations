use std::fs::remove_file;
use std::time::Duration;
use std::{path::Path, fs::File};
use std::io::Write;

struct FileLock;

impl FileLock {
    fn new() -> Self {
        let path = Path::new("file.lock");
        if path.exists() {
            panic!("You can't run this program more than once");
        }
        let mut output = File::create(path).unwrap();
        write!(output, "locked").unwrap();

        Self
    }
}

impl Drop for FileLock {
    fn drop(&mut self) {
        let path = Path::new("file.lock");
        remove_file(path).unwrap();
    }
}

fn main() {
    let _lock = FileLock::new();
    // Pretend to do something important
    std::thread::sleep(Duration::from_secs(30));
}
