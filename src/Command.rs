pub mod command {
	use std::fmt;

	#[derive(Debug,Clone)]
	pub struct Command {
		pub name: String,
		pub description: String,
		pub usage: String,
		pub special_args: (char,String),
	}
	impl Command {
		fn new(name: &str, description: &str, usage: &str, special_args: (char,&str)) -> Command {
			Command {
				name: name.to_string(),
				description: description.to_string(),
				usage: usage.to_string(),
				special_args: (special_args.0,special_args.1.to_string()),
			}
		}
	}
	impl fmt::Display for Command {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:>10} | {:>2}/ {:>6} | {}", self.name, self.special_args.0, self.special_args.1, self.description)
		}
	}

	pub(crate) fn get_commands() -> [Command; 5] {
		return [
			Command::new("new", "Creates a new shortcut", "<command> <directory_path> <shortcut_name>", ('n', "new")),
			Command::new("help", "Shows this list or helps with command", "<command>", ('h', "help")),
			Command::new("shortcuts", "Shows the shortcuts list", "<command>", ('s', "shortcuts")),
			Command::new("edit", "Edits a shortcut", "<command> <shortcut_name> <new_shortcut_name> <new_directory_path>", ('e', "edit")),
			Command::new("delete", "Deletes a shortcut", "<command> <shortcut_name|index|directory_path>", ('d', "del")),
		];
	}
}