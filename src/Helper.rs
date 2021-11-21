pub mod helpers{
	use serde::{Deserialize, Serialize};
	use serde_json::Result;
	use std::{env, fmt, fs};
	use std::process::exit;
	use std::path::{Path, PathBuf};

	#[derive(Serialize, Deserialize,Clone)]
	pub struct Record {
		pub name: String,
		pub path: String
	}

	impl fmt::Display for Record {
		fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
			write!(f,"{:>10} | {}",self.name, self.path)
		}
	}
	impl Record {
		pub fn new(name: String, path: String) -> Record {
			Record {
				name,
				path
			}
		}
	}
	pub trait Back {
		fn back(&self) -> PathBuf;
	}
	pub trait EnsurePathExists {
		fn ensure_path_exists(&self);
	}
	impl Back for Path {
		fn back(&self) -> PathBuf {
			let mut v = (&self).to_str().unwrap().split("\\").collect::<Vec<&str>>();
			v.remove(v.len() - 1);
			return Path::new(&v.join("\\")).to_path_buf();
		}
	}
	impl EnsurePathExists for Path {
		fn ensure_path_exists(&self){
			if !self.exists() {
				match fs::create_dir_all(self.back()) {
					Ok(_) => {
						match fs::write(self, if self.extension().unwrap().to_str().unwrap() == "json" {"[]"} else {""}) {
							Ok(_) => {},
							Err(e) => {
								println!("{}", e);
								exit(1)
							}
						}
					},
					Err(e) => {
						println!("{}", e);
						exit(1);
					}
				}
			};
		}
	}
	const RECORDS_FILE_PATH: &str = "config\\data.json";

	pub fn get_record_file_path() -> PathBuf {
		let path = env::current_exe().unwrap();
		return Path::new(&path).back().join(RECORDS_FILE_PATH);
	}
	pub fn get_file_contents() -> Vec<Record> {
		get_record_file_path().ensure_path_exists();
		match fs::read_to_string(get_record_file_path()) {
			Ok(data_buf) => {
				let data: Result<Vec<Record>> = serde_json::from_str(&data_buf);
				return data.unwrap();
			},
			Err(e) => {
				println!("{}", e);
				exit(1);
			}
		}
	}
	pub fn save_file_contents(data: &Vec<Record>) {
		let data = serde_json::to_string(&data).unwrap();
		fs::write(get_record_file_path(), data).expect("Unable to write file");
	}
	pub fn get_shortcut_names(data: &Vec<Record>) -> Vec<String> {
		let mut names: Vec<String> = Vec::new();
		for record in data {
			names.push(record.name.clone());
		}
		names
	}
	pub fn get_shortcut_paths(data: &Vec<Record>) -> Vec<String> {
		let mut paths: Vec<String> = Vec::new();
		for record in data {
			paths.push(record.path.clone());
		}
		paths
	}	
}