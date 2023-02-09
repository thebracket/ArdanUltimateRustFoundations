use thiserror::Error;

#[derive(Error, Debug)]
enum InputError {
    #[error("Standard input is unavailable")]
    StdIn,

    #[error("Cannot parse integer from text")]
    NotAnInteger,
}

fn get_line_from_keyboard() -> Result<String, InputError> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).map_err(|_| InputError::StdIn)?;
    let trimmed = input.trim();
    Ok(trimmed.to_string())
}

fn get_int_from_keyboard() -> Result<i32, InputError> {
    let text = get_line_from_keyboard()?;
    text.trim().parse().map_err(|_| InputError::NotAnInteger)
}

fn main() {
    loop {
        println!("Enter an integer:");
        let number = get_int_from_keyboard();
        match number {
            Ok(n)  => { println!("You entered {n}"); break; },
            Err(InputError::StdIn) => panic!("Input doesn't work"),
            Err(InputError::NotAnInteger) => println!("Please try again"),
        }
    }
}
