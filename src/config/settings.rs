use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Settings {
	pub scrolloff: u16,
	pub default_path: Option<PathBuf>,
	pub replace_txt_file_on_save: bool,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			scrolloff: 8,
			default_path: None,
			replace_txt_file_on_save: false,
		}
	}
}
