fn do_something(s: String) {
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    do_something(s);
    do_something(s);
}