use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug, Clone)]
pub struct Keys {
	pub quit: (KeyCode, KeyModifiers),
	pub cursor_down: (KeyCode, KeyModifiers),
	pub cursor_up: (KeyCode, KeyModifiers),
	pub cursor_top: (KeyCode, KeyModifiers),
	pub cursor_bottom: (KeyCode, KeyModifiers),
	pub seek_backwards: (KeyCode, KeyModifiers),
	pub seek_forwards: (KeyCode, KeyModifiers),
	pub toggle_pause: (KeyCode, KeyModifiers),
	pub volume_down: (KeyCode, KeyModifiers),
	pub volume_up: (KeyCode, KeyModifiers),
	pub volume_down_slightly: (KeyCode, KeyModifiers),
	pub volume_up_slightly: (KeyCode, KeyModifiers),
	pub speed_down: (KeyCode, KeyModifiers),
	pub speed_up: (KeyCode, KeyModifiers),
	pub speed_reset: (KeyCode, KeyModifiers),
}

impl Default for Keys {
	fn default() -> Self {
		Self {
			quit: (KeyCode::Char('q'), KeyModifiers::NONE),
			cursor_down: (KeyCode::Char('j'), KeyModifiers::NONE),
			cursor_up: (KeyCode::Char('k'), KeyModifiers::NONE),
			cursor_top: (KeyCode::Char('g'), KeyModifiers::NONE),
			cursor_bottom: (KeyCode::Char('G'), KeyModifiers::NONE),
			seek_backwards: (KeyCode::Char('H'), KeyModifiers::NONE),
			seek_forwards: (KeyCode::Char('L'), KeyModifiers::NONE),
			toggle_pause: (KeyCode::Char('r'), KeyModifiers::NONE),
			volume_down: (KeyCode::Char('['), KeyModifiers::NONE),
			volume_up: (KeyCode::Char(']'), KeyModifiers::NONE),
			volume_down_slightly: (KeyCode::Char('{'), KeyModifiers::NONE),
			volume_up_slightly: (KeyCode::Char('}'), KeyModifiers::NONE),
			speed_down: (KeyCode::Char('-'), KeyModifiers::NONE),
			speed_up: (KeyCode::Char('+'), KeyModifiers::NONE),
			speed_reset: (KeyCode::Char('='), KeyModifiers::NONE),
		}
	}
}
