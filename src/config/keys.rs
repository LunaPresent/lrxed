use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keys {
	pub quit: (KeyCode, KeyModifiers),
}

impl Default for Keys {
	fn default() -> Self {
		Self {
			quit: (KeyCode::Char('q'), KeyModifiers::empty()),
		}
	}
}
