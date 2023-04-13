use serde_derive::Deserialize;
use std::fs;
use thiserror::Error;
use toml;

#[derive(Error, Debug)]
pub enum ConfigReadError<'a> {
	#[error("A configuration file by the name of 'project.build' was not found in '{0}'")]
	NoConfigFile(&'a str),

	#[error("The configuration file at '{0}' could not be read because '{1}'")]
	InvalidConfigFile(&'a str, String),
}

#[derive(Deserialize, Clone)]
pub struct ProjectBuildConfigProject {
	pub name: String,
	pub owner: String,
	pub description: Option<String>,
	pub target_directory: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ProjectBuildConfigWorkspace {
	pub include: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct ProjectBuildConfig {
	pub project: ProjectBuildConfigProject,
	pub workspace: ProjectBuildConfigWorkspace,
}

pub fn read_project_file(directory: &str) -> Result<ProjectBuildConfig, ConfigReadError> {
	let raw_data = match fs::read_to_string(format!("{}project.build", directory)) {
		Ok(c) => c.replace("->", "="),
		Err(reason) => {
			return if reason.kind() == std::io::ErrorKind::NotFound {
				Err(ConfigReadError::NoConfigFile(directory))
			} else {
				Err(ConfigReadError::InvalidConfigFile(directory, reason.to_string()))
			}
		}
	};

	let mut raw_config: ProjectBuildConfig = match toml::from_str(&raw_data) {
		Ok(c) => c,
		Err(reason) => {
			return Err(ConfigReadError::InvalidConfigFile(directory, reason.to_string()))
		}
	};

	if raw_config.project.target_directory.is_none() {
		raw_config.project.target_directory = Some("/build/".to_string());
	}

	Ok(raw_config)
}