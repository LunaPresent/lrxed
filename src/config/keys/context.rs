use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter};

#[derive(Debug, Clone, Copy, PartialEq, EnumCount, EnumIter, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Context {
	Global,
	Editor,
	ConfirmBox,
	FileBrowser,
}
