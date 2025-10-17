use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::resource::Resource;
use oprabeli::ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};
use serde_with::{FromInto, serde_as};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MainColours {
	pub background: Color,
	pub overlay: Color,
	pub accent: Color,
	pub border_error: Color,
	pub border_info: Color,
	pub border_warn: Color,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MainStyles {
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub button_active: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub button_inactive: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub text_primary: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub text_secondary: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub title: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub cursor_line: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub lyrics_line: Style,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FileBrowserStyles {
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub parent_directory: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub directory: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub file: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub highlight_file: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub highlight_directory: Style,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct FileBrowserTheme {
	pub styles: FileBrowserStyles,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
#[serde(rename_all = "kebab-case")]
pub struct Theme {
	pub colours: MainColours,
	pub styles: MainStyles,
	pub file_browser: FileBrowserTheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StyleConfig {
	#[serde(alias = "foreground")]
	fg: Option<Color>,
	#[serde(alias = "background")]
	bg: Option<Color>,
	bold: Option<bool>,
	italic: Option<bool>,
}

fn modifier_to_opt_bool(style: Style, modifier: Modifier) -> Option<bool> {
	if style.add_modifier.contains(modifier) {
		Some(true)
	} else if style.sub_modifier.contains(modifier) {
		Some(false)
	} else {
		None
	}
}

impl From<Style> for StyleConfig {
	fn from(value: Style) -> Self {
		Self {
			fg: value.fg,
			bg: value.bg,
			bold: modifier_to_opt_bool(value, Modifier::BOLD),
			italic: modifier_to_opt_bool(value, Modifier::ITALIC),
		}
	}
}

impl From<StyleConfig> for Style {
	fn from(value: StyleConfig) -> Self {
		let mut add_modifier = Modifier::empty();
		let mut sub_modifier = Modifier::empty();
		if value.bold == Some(true) {
			add_modifier.insert(Modifier::BOLD);
		} else if value.bold == Some(false) {
			sub_modifier.insert(Modifier::BOLD);
		}
		if value.italic == Some(true) {
			add_modifier.insert(Modifier::ITALIC);
		} else if value.italic == Some(false) {
			sub_modifier.insert(Modifier::ITALIC);
		}

		Self {
			fg: value.fg,
			bg: value.bg,
			underline_color: None,
			add_modifier,
			sub_modifier,
		}
	}
}
