use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keys {
	pub quit: (KeyCode, KeyModifiers),
	pub cursor_down: (KeyCode, KeyModifiers),
	pub cursor_up: (KeyCode, KeyModifiers),
	pub cursor_top: (KeyCode, KeyModifiers),
	pub cursor_bottom: (KeyCode, KeyModifiers),
}

impl Default for Keys {
	fn default() -> Self {
		Self {
			quit: (KeyCode::Char('q'), KeyModifiers::NONE),
			cursor_down: (KeyCode::Char('j'), KeyModifiers::NONE),
			cursor_up: (KeyCode::Char('k'), KeyModifiers::NONE),
			cursor_top: (KeyCode::Char('g'), KeyModifiers::NONE),
			cursor_bottom: (KeyCode::Char('G'), KeyModifiers::SHIFT),
		}
	}
}
