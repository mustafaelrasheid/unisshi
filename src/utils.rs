use std::io::{stdin, Write, stdout};
use std::error::Error;

pub fn input(prompt: &str) -> Result<String, Box<dyn Error>> {
	let mut buffer = String::new();

	println!("{}", prompt);
	print!("> ");
	stdout().flush()?;
	match stdin().read_line(&mut buffer) {
		Ok(n) => {
			if n == 0 {
				return Err("unexpected input".into());
			}
		},
		Err(error) => {
			return Err(error.into());
		}
	}

	return Ok(buffer.trim().to_string());
}

pub fn input_y_or_n(prompt: &str, default: bool) -> Result<bool, Box<dyn Error>> {
	let formatted_prompt = if default {
		format!("{} [Y/n]", prompt)
	} else {
		format!("{} [y/N]", prompt)
	};
	
	loop {
		match input(&formatted_prompt) {
			Ok(text) => {
				match text.as_str() {
					"y" | "Y" => {
						return Ok(true);
					},
					"N" | "n" => {
						return Ok(false);
					},
					"" | "D" | "d" => {
						return Ok(default);
					},
					result => {
						eprintln!("{} is incorrectly formatted", result);
					}
				}
			},
			Err(error) => {
				return Err(error);
			}
		}
	}
}
