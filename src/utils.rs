use std::io::{stdin, Write, stdout};

pub fn input(prompt: &str) -> String {
    let mut buffer = String::new();

    println!("{}", prompt);
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut buffer).unwrap();

    return buffer.trim().to_string();
}

pub fn input_y_or_n(prompt: &str, default: bool) -> bool {
    let formatted_prompt = format!("{} [y/n]", prompt);

    loop {
        match input(&formatted_prompt).as_str() {
            "y" | "Y" => {
                return true;
            },
            "N" | "n" => {
                return false;
            },
            "" | "D" | "d" => {
                return default;
            }
            result => {
                eprintln!("{} is incorrectly formatted. try again", result);
            }
        }
    }
}
