pub mod command {
	use std::fmt;

	#[derive(Debug,Clone)]
	pub struct StaticCommand {
		pub name: String,
		pub description: String,
		pub usage: String,
		pub special_args: (String,String),
	}
	impl StaticCommand {
		fn new(name: &str, description: &str, usage: &str, special_args: (&str,&str)) -> StaticCommand {
			StaticCommand {
				name: name.to_string(),
				description: description.to_string(),
				usage: usage.to_string(),
				special_args: (special_args.0.to_string(),special_args.1.to_string()),
			}
		}
	}
	impl fmt::Display for StaticCommand {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f, "{:>10} | {:>2}/ {:>6} | {}", self.name, self.special_args.0, self.special_args.1, self.description)
		}
	}
	pub struct Command {
		pub args: Vec<String>,
		pub special_arg: String,
	}
	impl Command {
		fn new(&self, args: &Vec<String>, special_arg: &str) -> Command {
			Command {
				args: args.clone(),
				special_arg: special_arg.to_string(),
			}
		}
	}

	const STATIC_COMMANDS: [StaticCommand; 5] = [
		StaticCommand::new("new", "Creates a new shortcut", "<command> <directory_path> <shortcut_name>", ("n", "new")),
		StaticCommand::new("help", "Shows this list or helps with command", "<command>", ("h", "help")),
		StaticCommand::new("shortcuts", "Shows the shortcuts list", "<command>", ("l", "list")),
		StaticCommand::new("edit", "Edits a shortcut", "<command> <shortcut_name> <new_shortcut_name> <new_directory_path>", ("e", "edit")),
		StaticCommand::new("delete", "Deletes a shortcut", "<command> <shortcut_name|index|directory_path>", ("d", "del")),
	];
	pub(crate) fn get_static_commands() -> &'static Vec<StaticCommand> {
		&STATIC_COMMANDS.to_vec()
	}
}