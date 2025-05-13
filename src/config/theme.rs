use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
	pub cursorline: Style,
	pub current_line: Style,
}

impl Default for Theme {
	fn default() -> Self {
		Self {
			cursorline: Style::new().bg(Color::DarkGray),
			current_line: Style::new().add_modifier(Modifier::BOLD),
		}
	}
}
