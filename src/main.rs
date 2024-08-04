#![allow(unused_imports)]
use std::io::{self, Write};
use std::process::{Command, Stdio};

mod builtin;

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
		
		// Remove unnecessary newline at the end of the string
		if input.ends_with('\n') {
			input.pop();
		}

		// Parse the command into args
		// TODO: find a better way to do this
		let args = input.split_ascii_whitespace().map(|s| s.to_owned()).collect::<Vec<String>>();
		let command = &args[0];

		let mut output: String = String::new();
		let mut command_found = false;
		
		// Check if the command is a builtin
		for (cmd, func) in crate::builtin::COMMANDS.iter() {
			if command != *cmd {continue;}
			
			command_found = true;
			output = func(&args);
			break;
		}

		if command_found {
			println!("{}", output);
			continue;
		}

		// check if the command is an exe
		#[allow(non_snake_case)]
		let PATH = builtin::get_env_path();
		if PATH.is_none() {
			println!("{}", no_command_found(&input));
			continue;
		}

		let mut cmd_status: Option<Result<std::process::ExitStatus, io::Error>> = None;

		for exe_path in PATH.unwrap() {
			let cmd_path = exe_path.join(command);
			if !cmd_path.exists() {continue;}

			cmd_status = Some(Command::new(cmd_path)
				.args(&args[1..])
				.status());
			break;
		}

		if cmd_status.is_none() {
			println!("{}", no_command_found(&input));
			continue;
		}

		let cmd_result = cmd_status.unwrap();

		if cmd_result.is_err() {
			println!("Failed to run command {}: {}", command, cmd_result.unwrap_err().to_string());
		}

		continue;
	}
}
