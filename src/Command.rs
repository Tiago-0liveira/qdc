pub mod command {
	use std::fmt;
	use colored::*;
	#[derive(Debug,Clone)]
	pub struct Command {
		pub name: String,
		pub description: String,
		pub usage: String,
		pub special_args: (char,String),
		pub args_num: usize
	}
	impl Command {
		fn new(name: &str, description: &str, usage: &str, special_args: (char,&str), args_num: usize) -> Command {
			Command {
				name: name.to_string(),
				description: description.to_string(),
				usage: usage.to_string(),
				special_args: (special_args.0,special_args.1.to_string()),
				args_num
			}
		}
	}
	impl fmt::Display for Command {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			let v_bar = "|".truecolor(120,120,120);
			write!(f," {:10} {} {:>1}\\{:>11} {} {}",&self.name.cyan(), &v_bar, "-".to_string() + &self.special_args.0.to_string(), "--".to_string()+&self.special_args.1, v_bar, &self.description)
		}
	}

	impl Command {
		pub fn show_Usage(&self) {
			println!("\n|{:^16}: {}","Help for command", self.name.cyan().bold());
			println!("|{:^16}: {}","Description".green(), self.description);
			println!("|{:^16}: {}","Usage".blue(), self.usage);
		}
	}

	pub(crate) fn get_commands() -> [Command; 5] {
		return [
			Command::new("new", "Creates a new shortcut", "<new|-n|--new> <directory_path> <shortcut_name>", ('n', "new"), 2),
			Command::new("help", "Shows the commands list or helps with a specific command", "<help|-h|--help> <?command_name>", ('h', "help"), 0),
			Command::new("shortcuts", "Shows the shortcuts list", "<shortcuts|-s|--shortcuts>", ('s', "shortcuts"), 0),
			Command::new("edit", "Edits a shortcut", "<edit|-e|--edit> <shortcut_name> <new_shortcut_name> <new_directory_path>", ('e', "edit"), 3),
			Command::new("delete", "Deletes a shortcut", "<delete|-d|--del> <shortcut_name|index|directory_path>", ('d', "del"), 1),
		];
	}
}