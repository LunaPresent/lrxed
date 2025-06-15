use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumIter, IntoStaticStr};

#[derive(
	Debug, Clone, Copy, PartialEq, EnumCount, EnumIter, IntoStaticStr, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub enum Context {
	Global,
	ConfirmBox,
	ScrollablePopup,
	Editor,
	FileBrowser,
}
