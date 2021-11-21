#![allow(non_snake_case)]
mod Command;
mod Helper;

use crate::Command::command;
use crate::Helper::helpers;
use std;
use std::env;
use std::path::Path;
use std::process::exit;
use colored::*;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if (&args).len() > 1 {
        process_cli_args(args);
    } else {
        show_help();
    }
}

fn show_help() {
    let coms = &command::get_commands();
    print!("\n   --> {}\n", "QDC".green().bold());
    coms.iter().for_each(|c| {
        println!("{}", c);
    });
}

fn process_cli_args(args: Vec<String>) {
    let coms = &command::get_commands();
    let first_arg = (&args)[1].trim_start_matches("-").trim().to_string();
    let com = coms.iter().find(|c|
        c.name == (&first_arg).to_string() ||
        c.special_args.1 == (&first_arg).to_string() ||
        c.special_args.0.to_string() == (&first_arg).to_string()
    );
    if com.is_none() { 
        println!("Command {} not found", first_arg.red());
        println!("Use qdc help to see available commands!");
        exit(1);
    }
    let com = com.unwrap();
    if (&com).args_num > (&args).len() - 2 {
        println!("Command {} requires {} arguments, args num: {}, provided args: {:?}", (&com).name.cyan(), (&com).args_num.to_string().green(), (args.len() - 2).to_string().red(), args[2..].to_vec());
        exit(1);
    }
    let args = (&args)[2..].to_vec();
    let mut file_contents = helpers::get_file_contents();
    let shortcutNames = helpers::get_shortcut_names(&file_contents);
    let shortcutPaths = helpers::get_shortcut_paths(&file_contents);
    match com.name.as_str() {
        "new" => {
            if !args[0].trim().is_empty() {
                let shortcutName = args[0].trim();
                if !shortcutNames.contains(&shortcutName.to_string()) {
                    let path = args[1].trim();
                    if !shortcutPaths.contains(&path.to_string()) {
                        if Path::new(&path).exists() {
                            if Path::new(&path).is_dir() {
                                file_contents.push(helpers::Record::new((&shortcutName).to_string(), (&path).to_string()));
                                helpers::save_file_contents(file_contents);
                                println!("Shortcut {} created", shortcutName.green());
                            } else {
                                println!("Path {} is not a directory", path.red());
                                exit(1);
                            }
                        } else {
                            println!("Path {} is invalid!", path.red());
                            exit(1);
                        }
                    } else {
                        println!("Shortcut {} already exists", shortcutName.red());
                        exit(1);
                    }
                } else {
                    println!("Shortcut {} already exists", shortcutName.red());
                    exit(1);
                }
            } else {
                println!("Shortcut name is required");
                exit(1);
            }
        },
        "help" => {
            if args.len() != 0 && !args[0].trim().is_empty() {
                let comName = args[0].trim();
                let com = coms.iter().find(|c|
                    c.name == (&comName).to_string() ||
                    c.special_args.1 == (&comName).to_string() ||
                    c.special_args.0.to_string() == (&comName).to_string()
                );
                if com.is_none() { 
                    println!("Command {} not found", comName.red());
                    println!("Use qdc help to see available commands!");
                    exit(1);
                } else {
                    let com = com.unwrap();
                    com.show_Usage();
                }
            } else {
                show_help();
            }
        },
        "shortcuts" => {},
        "edit" => {},
        "delete" => {},
        _ => {
            println!("Command {} not found!", first_arg.red());
            exit(1);
        }
    };
}