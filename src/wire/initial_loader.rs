use std::process::exit;
use super::config::{read_project_file, ConfigReadError};
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

pub fn handle() {
	let mut sp = Spinner::new(Spinners::Dots9, "Attempting to load configuration file".into());
	let command_line_args: Vec<String> = std::env::args().collect();
	let file_path = if command_line_args.len() > 1 {
		command_line_args[1].clone()
	} else {
		sp.stop_with_message("No configuration file path provided".into());
		exit(1);
	};

	let config = match read_project_file(&file_path) {
		Ok(c) => c,
		Err(e) => {
			sp.stop_with_message(format!("Error: {}", e));
			exit(1);
		}
	};

	sp.stop_with_message(format!("Loaded configuration for '{}@{}' from '{}project.build'", config.project.name, config.project.owner, file_path));
}