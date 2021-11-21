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
    let mut file_contents = helpers::get_file_contents();
    let shortcutNames = helpers::get_shortcut_names(&file_contents);
    let shortcutPaths = helpers::get_shortcut_paths(&file_contents);
    if com.is_none() {
        if shortcutNames.contains(&first_arg) {
            let w = file_contents.iter().find(|s| s.name == (&first_arg).to_string()).unwrap();
            println!("{}", &w.path);

            env::set_current_dir(Path::new(&w.path)).expect("Failed to set current directory");
            exit(0);
        } else {
            println!("Command {} not found", first_arg.red());
            println!("Use qdc help to see available commands!");
            exit(1);
        }
    }
    let com = com.unwrap();
    if (&com).args_num > (&args).len() - 2 {
        println!("Command {} requires {} arguments, args num: {}, provided args: {:?}", (&com).name.cyan(), (&com).args_num.to_string().green(), (args.len() - 2).to_string().red(), args[2..].to_vec());
        exit(1);
    }
    let args = (&args)[2..].to_vec();
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
                                helpers::save_file_contents(&file_contents);
                                println!("Shortcut {} created with path: {}", shortcutName.green(), (&path).to_string().cyan());
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
        "shortcuts" => {
            if file_contents.len() != 0 {
                println!("{:>10} | {}", "name".bold().blue(), "path".bold().green());
                file_contents.iter().for_each(|s| {
                    println!("{}", s);
                });
            } else {
                println!("No shortcuts {}!", "found".red());
            }
        },
        "edit" => {
            let rec = (&file_contents).into_iter().position(|c|
                c.name.trim_start_matches("-") == (&args[0]).to_string().trim_start_matches("-")
            );
            if !rec.is_none() {
                let rec_index = (&file_contents).into_iter().position(|c|
                    c.name.trim_start_matches("-") == (&args[0]).to_string().trim_start_matches("-")
                );
                if &args[1] == "~" && &args[2] == "~" {println!("Are you {}", "dumb?".bold().red()); exit(1);}
                if Path::new(&args[2]).exists() {
                    if Path::new(&args[2]).is_dir() {
                        file_contents[rec_index.unwrap()] = helpers::Record::new(args[1].trim().to_string(), args[2].trim().to_string());
                        helpers::save_file_contents(&file_contents);
                    } else {
                        println!("Path {} is not a directory", args[2].red());
                        exit(1);
                    }
                } else {
                    println!("Path {} is invalid!", args[2].red());
                    exit(1);
                }
            } else {
                println!("Shortcut {} not found", args[0].red());
                exit(1);
            }
        },
        "delete" => {
            let rec: Option<&helpers::Record> = file_contents.iter().find(|c|
                c.name == (&args[0]).to_string().trim_start_matches("-")
            );
            if !rec.is_none() {
                let record = &rec.unwrap();
                let rec_index = file_contents.iter().position(|s| s.name == record.name);
                let mut w = file_contents.clone();
                w.remove(rec_index.unwrap());
                helpers::save_file_contents(&w);
                println!("Shortcut {} deleted!", record.name.green());
            } else {
                println!("Shortcut {} not found", args[0].red());
                exit(1);
            }
        },
        _ => {
            println!("Command {} not found!", first_arg.red());
            exit(1);
        }
    };
}