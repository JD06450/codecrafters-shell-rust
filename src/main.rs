mod builtin;

#[allow(unused_imports)]
use std::io::{self, Write};

fn no_command_found(input: &String) -> String {
	format!("{}: command not found", input)
}

fn main() {
	loop {
		print!("$ ");
		io::stdout().flush().unwrap();
		
		// Wait for user input
		let stdin = io::stdin();
		let mut input = String::new();
		stdin.read_line(&mut input).unwrap();
		
		if input.ends_with('\n') {
			input.pop();
		}

		let args = input.split_ascii_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
		let command = &args[0];

		let mut output: String = String::new();
		let mut command_found = false;
		
		for (cmd, func) in crate::builtin::COMMANDS.iter() {
			if command == *cmd {
				command_found = true;
				output = func(&args);
				break;
			}
		}

		if command_found {
			println!("{}", output);
		} else {
			println!("{}", no_command_found(&input));
		}
	}
}
