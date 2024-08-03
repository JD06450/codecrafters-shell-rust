#[allow(unused_imports)]
use std::io::{self, Write};
// use std::fmt;

fn no_command_found(input: &String) -> String
{
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
		
		let output: String;
		
		match input {
			_ => output = no_command_found(&input)
		}
		println!("{}", output);
	}
}
