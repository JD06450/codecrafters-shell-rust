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

pub fn exit(args: &Vec<String>) -> String
{
	if args.len() < 2 {std::process::exit(0);}

	let exit_code = args[1].parse::<i32>();
	std::process::exit(exit_code.unwrap_or_default());
}

pub fn echo (args: &Vec<String>) -> String
{
	let mut printed = args.clone();
	printed.remove(0);

	printed.join(" ")
}

pub fn cmd_type (args: &Vec<String>) -> String
{
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

pub static COMMANDS: [(&'static str, fn(&Vec<String>) -> String); 3] = [
	("exit", exit),
	("echo", echo),
	("type", cmd_type)
];

lazy_static! {
	pub static ref COMMAND_NAMES: [&'static str; 3] = COMMANDS.map(|c| c.0);
}