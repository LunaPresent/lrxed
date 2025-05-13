use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
	pub cursorline: Style,
	pub lyrics_line: Style,
}

impl Default for Theme {
	fn default() -> Self {
		Self {
			cursorline: Style::new().bg(Color::Rgb(65, 69, 89)),
			lyrics_line: Style::new().add_modifier(Modifier::BOLD),
		}
	}
}
