use anyhow::Result;

fn get_line_from_keyboard() -> Result<String> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}

fn get_int_from_keyboard() -> Result<i32> {
    let text = get_line_from_keyboard()?;
    Ok(text.trim().parse()?)
}

fn main() {
    loop {
        println!("Enter an integer:");
        let number = get_int_from_keyboard();
        match number {
            Ok(n)  => { println!("You entered {n}"); break; },
            Err(e) => {
                if let Some(std::io::Error { .. }) = e.downcast_ref::<std::io::Error>() {
                    panic!("stdin is unavailable");
                } else {
                    println!("Try again - that wasn't an integer");
                }
            }
        }
    }
}
