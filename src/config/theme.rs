use ratatui::style::{Modifier, Style, Stylize};

#[derive(Debug, Clone)]
pub struct Theme {
	pub button_active: Style,
	pub button_inactive: Style,
	pub confirm_box: Style,
	pub cursorline: Style,
	pub lyrics_line: Style,
}

impl Default for Theme {
	fn default() -> Self {
		Self {
			button_active: Style::new().blue().on_black(),
			button_inactive: Style::new().on_black(),
			confirm_box: Style::new().yellow(),
			cursorline: Style::new().on_black(),
			lyrics_line: Style::new().add_modifier(Modifier::BOLD),
		}
	}
}
