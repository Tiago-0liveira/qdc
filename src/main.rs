#![allow(non_snake_case)]
mod Command;

use Command::command::StaticCommand;

use crate::Command::command;
use std;
use std::io::{Write, stdin, stdout};
use std::env;
use std::process::exit;

fn main() {
    if env::args().len() > 1 {
        for argument in env::args().skip(1) {
            process_cli_arg(argument)        
        }
    } else {
        let mut input = String::new();
        let quit = String::from("quit");
        while !&input.trim().to_lowercase().ends_with(&quit) {
            print!("> ");
            let _ = stdout().flush();
            match stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(error) => {
                    println!("Error: {}", error);
                    exit(1)
                },
            }
        }
    }
}

fn show_help() {
    command::get_static_commands().iter().for_each(|c| {
        println!("{}", c);
    });
}

fn process_cli_arg(arg: String) {
    let command = command::get_static_commands().iter().find(|c| c.name == arg);
    if let Some(command) = command {
        println!("arg: {}", arg);
        if arg.starts_with("-n:") || arg.starts_with("--new:") {
            let new_line = arg.split(":").nth(1).unwrap();
            println!("parsed arg -> {}", new_line);
        } else if arg.starts_with("-h") || arg.starts_with("--help") {
            show_help();
            exit(0);   
        } else if arg.starts_with("-l") || arg.starts_with("--list") {
            println!("-l --list");
            exit(0);
        }
    } else {
        println!("Command not found: {}", arg);
    }
}