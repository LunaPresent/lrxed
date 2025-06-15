use ratatui::style::{Color, Modifier, Style, Stylize};
use serde::{Deserialize, Serialize};
use serde_with::{FromInto, serde_as, skip_serializing_none};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Theme {
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub accent: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub border_info: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub border_warn: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub button_active: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub button_inactive: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub cursorline: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub inactive: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub lyrics_line: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub text_secondary: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub title: Style,

	#[serde_as(as = "FromInto<StyleConfig>")]
	pub file_browser_parent_directory: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub file_browser_directory: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub file_browser_file: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub file_browser_highlight_directory: Style,
	#[serde_as(as = "FromInto<StyleConfig>")]
	pub file_browser_highlight_file: Style,
}

impl Default for Theme {
	fn default() -> Self {
		Self {
			accent: Style::new().magenta(),
			border_info: Style::new().blue(),
			border_warn: Style::new().yellow(),
			button_active: Style::new().blue().on_black(),
			button_inactive: Style::new().on_black(),
			cursorline: Style::new().on_black(),
			inactive: Style::new().black(),
			lyrics_line: Style::new().bold(),
			text_secondary: Style::new().dark_gray(),
			title: Style::new().magenta().bold(),

			file_browser_parent_directory: Style::new().green(),
			file_browser_directory: Style::new().blue(),
			file_browser_file: Style::new().reset(),
			file_browser_highlight_file: Style::new().black().on_white(),
			file_browser_highlight_directory: Style::new().black().on_blue(),
		}
	}
}

#[skip_serializing_none]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
struct StyleConfig {
	#[serde(alias = "foreground")]
	fg: Option<ColourConfig>,
	#[serde(alias = "background")]
	bg: Option<ColourConfig>,
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
			fg: value.fg.map(|c| c.into()),
			bg: value.bg.map(|c| c.into()),
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
			fg: value.fg.map(|c| c.into()),
			bg: value.bg.map(|c| c.into()),
			underline_color: None,
			add_modifier,
			sub_modifier,
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ColourConfig {
	#[default]
	Reset,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	Gray,
	DarkGray,
	LightRed,
	LightGreen,
	LightYellow,
	LightBlue,
	LightMagenta,
	LightCyan,
	White,
	#[serde(untagged)]
	Rgb {
		r: u8,
		g: u8,
		b: u8,
	},
	#[serde(untagged)]
	Indexed(u8),
}

impl From<Color> for ColourConfig {
	fn from(value: Color) -> Self {
		match value {
			Color::Reset => Self::Reset,
			Color::Black => Self::Black,
			Color::Red => Self::Red,
			Color::Green => Self::Green,
			Color::Yellow => Self::Yellow,
			Color::Blue => Self::Blue,
			Color::Magenta => Self::Magenta,
			Color::Cyan => Self::Cyan,
			Color::Gray => Self::Gray,
			Color::DarkGray => Self::DarkGray,
			Color::LightRed => Self::LightRed,
			Color::LightGreen => Self::LightGreen,
			Color::LightYellow => Self::LightYellow,
			Color::LightBlue => Self::LightBlue,
			Color::LightMagenta => Self::LightMagenta,
			Color::LightCyan => Self::LightCyan,
			Color::White => Self::White,
			Color::Rgb(r, g, b) => Self::Rgb { r, g, b },
			Color::Indexed(i) => Self::Indexed(i),
		}
	}
}

impl From<ColourConfig> for Color {
	fn from(value: ColourConfig) -> Self {
		match value {
			ColourConfig::Reset => Self::Reset,
			ColourConfig::Black => Self::Black,
			ColourConfig::Red => Self::Red,
			ColourConfig::Green => Self::Green,
			ColourConfig::Yellow => Self::Yellow,
			ColourConfig::Blue => Self::Blue,
			ColourConfig::Magenta => Self::Magenta,
			ColourConfig::Cyan => Self::Cyan,
			ColourConfig::Gray => Self::Gray,
			ColourConfig::DarkGray => Self::DarkGray,
			ColourConfig::LightRed => Self::LightRed,
			ColourConfig::LightGreen => Self::LightGreen,
			ColourConfig::LightYellow => Self::LightYellow,
			ColourConfig::LightBlue => Self::LightBlue,
			ColourConfig::LightMagenta => Self::LightMagenta,
			ColourConfig::LightCyan => Self::LightCyan,
			ColourConfig::White => Self::White,
			ColourConfig::Rgb { r, g, b } => Self::Rgb(r, g, b),
			ColourConfig::Indexed(i) => Self::Indexed(i),
		}
	}
}
