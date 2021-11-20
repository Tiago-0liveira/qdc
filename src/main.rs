#![allow(non_snake_case)]
mod Command;

use crate::Command::command;
use std;
use std::env::{self, Args};
use std::process::exit;

fn main() {
    if env::args().len() > 1 {
        process_cli_args(env::args());
    } else {
        show_help();
    }
}

fn show_help() {
    let coms = &command::get_commands();
    coms.iter().for_each(|c| {
        println!("{}", c);
    });
}

fn process_cli_args(args: Args) {
    let coms = &command::get_commands();
    let mut first_arg = args.skip(1).next().unwrap();
    first_arg = (&first_arg.trim_start_matches("-")).to_string();
    let com = coms.iter().find(|c|  
        c.name == (&first_arg).to_string() ||
        c.special_args.1 == (&first_arg).to_string() ||
        c.special_args.0.to_string() == (&first_arg).to_string()
    );
    if com.is_none() {  
        println!("Command {} not found", first_arg);
        exit(1);
    }
    let com = com.unwrap();
    println!("{}", com);
}