use core::time::Duration;
use std::path::PathBuf;

use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::resource::Resource;
use serde::{Deserialize, Serialize};
use serde_with::{DurationSecondsWithFrac, chrono, serde_as};

#[serde_as]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Resource)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
	pub scrolloff: u16,
	pub default_path: Option<PathBuf>,
	pub replace_txt_file_on_save: bool,
	#[serde_as(as = "DurationSecondsWithFrac<f64>")]
	pub notification_timeout: Duration,
	#[serde_as(as = "DurationSecondsWithFrac<f64>")]
	pub sync_offset: chrono::TimeDelta,
}
