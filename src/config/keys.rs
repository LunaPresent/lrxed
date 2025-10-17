use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::resource::Resource;
use oprabeli::config::KeyConfig;
use serde::{Deserialize, Serialize};

use super::input_action::InputAction;

#[derive(Debug, Serialize, Deserialize, Resource)]
#[serde(rename_all = "kebab-case")]
pub struct Keys {
	pub global: KeyConfig<InputAction>,
	pub editor: KeyConfig<InputAction>,
	pub file_browser: KeyConfig<InputAction>,
	pub confirm_box: KeyConfig<InputAction>,
}
