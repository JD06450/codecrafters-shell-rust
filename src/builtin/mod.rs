#![allow(unused_imports)]
use std::vec;
use std::process;
use lazy_static::lazy_static;

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