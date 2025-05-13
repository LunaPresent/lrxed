use ratatui::style::{Color, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
	pub cursorline: Style,
}

impl Default for Theme {
	fn default() -> Self {
		Self {
			cursorline: Style::new().bg(Color::DarkGray),
		}
	}
}
