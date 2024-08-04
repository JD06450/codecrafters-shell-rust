#![allow(unused_imports)]
use lazy_static::lazy_static;
use std::env::{self};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use std::vec;

pub fn get_env_path() -> Option<Vec<PathBuf>> {
	match env::var_os("PATH") {
		Some(value) => Some(value.to_string_lossy().split(":").map(|s| PathBuf::from_str(s).unwrap()).collect()),
		None => None
	}
}

pub fn exit(args: &Vec<String>) -> String {
	if args.len() < 2 {std::process::exit(0);}

	let exit_code = args[1].parse::<i32>();
	std::process::exit(exit_code.unwrap_or_default());
}

pub fn echo(args: &Vec<String>) -> String {
	let mut printed = args.clone();
	printed.remove(0);

	printed.join(" ")
}

pub fn cmd_type(args: &Vec<String>) -> String {
	let str_to_find = &args[1];
	for cmd in *COMMAND_NAMES {
		if str_to_find == cmd {return format!("{} is a shell builtin", cmd);}
	}

	#[allow(non_snake_case)]
	let PATH = get_env_path();

	if PATH.is_some() {
		for exe_path in PATH.unwrap() {
			let cmd_path = exe_path.join(str_to_find);
			if cmd_path.exists() {
				return format!("{} is {}", str_to_find, cmd_path.display())
			}
		}
	}

	format!("{}: not found", str_to_find)
}

pub fn pwd(_args: &Vec<String>) -> String {
	env::current_dir().unwrap().display().to_string()
}

pub fn cd(args: &Vec<String>) -> String {
	let new_dir = PathBuf::from(args[1].clone());
	match env::set_current_dir(&new_dir) {
		Ok(_) => String::new(),
		Err(e) => {
			let error_string = e.to_string();
			let trunc_idx = error_string.find(" (").unwrap_or(error_string.len());

			format!("cd: {}: {}", new_dir.display(), error_string[0..trunc_idx].to_owned())
		}
	}
}

pub static COMMANDS: [(&'static str, fn(&Vec<String>) -> String); 5] = [
	("exit", exit),
	("echo", echo),
	("type", cmd_type),
	("pwd", pwd),
	("cd", cd)
];

lazy_static! {
	pub static ref COMMAND_NAMES: [&'static str; 5] = COMMANDS.map(|c| c.0);
}