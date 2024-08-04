#![allow(unused_imports)]
use std::vec;
use std::process;

pub fn exit(args: &Vec<String>) -> String
{
	if args.len() < 2 {std::process::exit(0);}

	let exit_code = args[1].parse::<i32>();
	std::process::exit(exit_code.unwrap_or_default());
}



pub static COMMANDS: [(&'static str, fn(&Vec<String>) -> String); 1] = [
	("exit", exit)
];