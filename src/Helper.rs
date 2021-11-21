pub mod helpers{
	use serde::{Deserialize, Serialize};
	use serde_json::Result;
	use std::{env, fs};
	use std::process::exit;
	use std::path::Path;

	#[derive(Serialize, Deserialize)]
	pub struct Record {
		pub name: String,
		pub path: String
	}

	impl Record {
		pub fn new(name: String, path: String) -> Record {
			Record {
				name,
				path
			}
		}
	}
	const RECORDS_FILE_PATH: &str = "config\\data.json";

	pub fn get_record_file_path() -> String {
		let path = env::args().next().unwrap();
		let v = path.split("\\").collect::<Vec<&str>>();
		return v[..(v.len()-1)].join("\\")+"\\"+RECORDS_FILE_PATH;
	}
	pub fn get_file_contents() -> Vec<Record> {
		match fs::read_to_string(get_record_file_path()) {
			Ok(data) => {
				let data: Result<Vec<Record>> = serde_json::from_str(&data);
				match data {
					Ok(data) => return data,
					Err(err) => {
						println!("{}", err);
						exit(1);
					}
				}
			},
			Err(err) => {
				if err.kind() == std::io::ErrorKind::NotFound {
					let path = env::args().next().unwrap();
					return match fs::write(get_record_file_path(), "[]") {
						Err(_) => {
							let v = path.split("\\").collect::<Vec<&str>>();
							match fs::create_dir(v[..(v.len()-1)].join("\\")+"\\config") {
								Ok(_) => Vec::new(),
								Err(err) => {
									match err.kind() {
										std::io::ErrorKind::AlreadyExists => {
											match fs::write(get_record_file_path(), "[]") {
												Ok(_) => Vec::new(),
												Err(err) => {
													println!("{}", err);
													exit(1);
												}
											}
										},
										_ => {
											println!("{}", err);
											exit(1);
										}
									}
								}
							}
						},
						Ok(_) => Vec::new()
					}
				} else {
					println!("{}", err);
					exit(1);
				}
			}
		};
	}
	pub fn save_file_contents(data: Vec<Record>) {
		let data = serde_json::to_string(&data).unwrap();
		fs::write(Path::new(RECORDS_FILE_PATH), data).expect("Unable to write file");
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