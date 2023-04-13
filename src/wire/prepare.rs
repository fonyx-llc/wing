use std::process::exit;
use crate::wing::config::ProjectBuildConfig;

pub fn prepare_for_build(config: ProjectBuildConfig, root_directory: &str) {
	let target_directory = config.project.target_directory.unwrap();
	if !std::path::Path::new(&format!("{}{}", root_directory, target_directory)).exists() {
		match std::fs::create_dir(format!("{}{}", root_directory, target_directory)) {
			Ok(_) => (),
			Err(e) => {
				println!("Error: {}", e);
				exit(1);
			}
		}
	}

	if !std::path::Path::new(&format!("{}{}target", root_directory, target_directory)).exists() {
		match std::fs::create_dir(format!("{}{}target", root_directory, target_directory)) {
			Ok(_) => (),
			Err(e) => {
				println!("Error: {}", e);
				exit(1);
			}
		}
	}
}