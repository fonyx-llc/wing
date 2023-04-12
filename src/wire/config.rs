use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum ConfigReadError<'a> {
	#[error("A configuration file by the name of 'project.build' was not found in '{0}'")]
	NoConfigFile(&'a str),

	#[error("The configuration file at '{0}' could not be read because '{1}'")]
	InvalidConfigFile(&'a str, String),
}

#[derive(Deserialize)]
pub struct ProjectBuildConfigProject {
	pub name: String,
	pub owner: String,
	pub description: Option<String>,
	pub intention: String,
}

#[derive(Deserialize)]
pub struct ProjectBuildConfig {
	pub project: ProjectBuildConfigProject,
}

pub fn read_project_file(directory: &str) -> Result<ProjectBuildConfig, ConfigReadError> {
	let raw_data = match fs::read_to_string(format!("{}project.build", directory)) {
		Ok(c) => c,
		Err(reason) => {
			return if reason.kind() == std::io::ErrorKind::NotFound {
				Err(ConfigReadError::NoConfigFile(directory))
			} else {
				Err(ConfigReadError::InvalidConfigFile(directory, reason.to_string()))
			}
		}
	};

	let raw_config: ProjectBuildConfig = match toml::from_str(&raw_data) {
		Ok(c) => c,
		Err(reason) => {
			return Err(ConfigReadError::InvalidConfigFile(directory, reason.to_string()))
		}
	};

	Ok(raw_config)
}