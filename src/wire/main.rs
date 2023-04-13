use std::process::exit;
use std::sync::Mutex;
use super::config::read_project_file;
use spinners::{Spinner, Spinners};
use crate::wire::prepare::prepare_for_build;

#[derive(PartialEq)]
pub enum Job {
	BuildToTarget,
	RunTarget,
	EmulateTarget,
	EmulateTest,
	RunTest,
	BuildAndRunTarget,
	BuildAndTest,
}

pub fn handle() {
	println!("\n| ---   ---   ---   ---   ---   ---   ---   ---   ---   ---   ---   --- |");
	println!("|                                                                       |");
	println!("| Fonyx Wire vBranch. (c) 2020 Fonyx Software. All rights reserved.     |");
	println!("| Welcome to Wire Server, a mimic of the Fonyx Unity build system.      |");
	println!("|                                                                       |");
	println!("| NOTICE: This notice will always appear, it's not a friendly welcome,  |");
	println!("| its to prevent bugs for bootstrap builds running on the Fonyx         |");
	println!("| Lighting processor and architecture.                                  |");
	println!("|                                                                       |");
	println!("| ---   ---   ---   ---   ---   ---   ---   ---   ---   ---   ---   --- |\n");

	let mut config_load_spinner = Spinner::new(Spinners::Dots9, "Attempting to load configuration file".into());
	let mut project_build_log: Mutex<String> = Mutex::new(String::new());
	let command_line_args: Vec<String> = std::env::args().collect();

	let build_log = |text: &str| {
		let mut string_item = project_build_log.lock().unwrap();
		*string_item = format!("{}[info] {}\n", string_item, text);
	};

	let build_error = |text: &str| {
	};

	let build_warning = |text: &str| {
	};

	let build_success = |text: &str| {
	};

	let (job_raw, file_path) = if command_line_args.len() > 2 {
		(command_line_args[1].clone(), command_line_args[2].clone())
	} else {
		config_load_spinner.stop_with_message("Insufficient arguments provided, Usage: wire <job> <root-directory>".into());
		exit(1);
	};

	let job: Job = match job_raw.as_str() {
		"build-to-target" => Job::BuildToTarget,
		"run-target" => Job::RunTarget,
		"run-test" => Job::RunTest,
		"emulate-target" => Job::EmulateTarget,
		"emulate-test" => Job::EmulateTest,
		"build-and-run-target" => Job::BuildAndRunTarget,
		"build-and-test" => Job::BuildAndTest,
		_ => {
			config_load_spinner.stop_with_message(format!("Invalid job (build-to-target | run-target | run-test | emulate-target | emulate-test | build-and-run-target | build-and-test) '{}', Usage: wire <job> <root-directory>", job_raw));
			exit(1);
		}
	};

	let config = match read_project_file(&file_path) {
		Ok(c) => c,
		Err(e) => {
			config_load_spinner.stop_with_message(format!("Error: {}", e));
			exit(1);
		}
	};

	config_load_spinner.stop_with_message(format!("-- Loaded configuration for '{}@{}' from '{}project.build'", config.project.name, config.project.owner, file_path));
	let target_directory = config.clone().project.target_directory.unwrap();

	if job == Job::BuildToTarget || job == Job::BuildAndRunTarget || job == Job::BuildAndTest {
		build_log("Preparing project for build");
		let mut preparation_spinner = Spinner::new(Spinners::Dots9, "-- Preparing project for build".into());
		prepare_for_build(config, &file_path);
		preparation_spinner.stop_with_message("-- Prepared project for build".into());

	} else if job == Job::RunTarget {
		let mut preparation_spinner = Spinner::new(Spinners::Dots9, "-- Ensuring run conditions are met".into());
		prepare_for_build(config, &file_path);
		preparation_spinner.stop_with_message("-- Ensured run conditions are met".into());
	}

	if job == Job::BuildToTarget || job == Job::BuildAndRunTarget || job == Job::BuildAndTest {
		let mut build_log_spinner = Spinner::new(Spinners::Dots9, "-- Writing build log information".into());
		let build_log = project_build_log.lock().unwrap().clone();

		match std::fs::write(format!("{}{}build.log", file_path, target_directory), build_log) {
			Ok(_) => build_log_spinner.stop_with_message("-- Wrote build log information".into()),
			Err(e) => {
				build_log_spinner.stop_with_message(format!("Error: {}", e));
				exit(1);
			}
		}
	}
}