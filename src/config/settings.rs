use serde::{Deserialize, Serialize};
use serde_with::{DurationSecondsWithFrac, serde_as};
use std::{path::PathBuf, time::Duration};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Settings {
	pub scrolloff: u16,
	pub default_path: Option<PathBuf>,
	pub replace_txt_file_on_save: bool,
	#[serde_as(as = "DurationSecondsWithFrac<f64>")]
	pub notification_timeout: Duration,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			scrolloff: 8,
			default_path: None,
			replace_txt_file_on_save: false,
			notification_timeout: Duration::from_secs(5),
		}
	}
}
