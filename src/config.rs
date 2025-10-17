mod config_manager;
mod input_action;
mod keys;
mod settings;
mod theme;

pub use config_manager::ConfigManager;
pub use keys::Keys;
pub use settings::Settings;
pub use theme::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct AppConfig {
	keys: Keys,
	theme: Theme,
	settings: Settings,
}
