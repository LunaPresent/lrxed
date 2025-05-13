use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyChord {
	pub key: KeyCode,
	pub mods: KeyModifiers,
}

impl KeyChord {
	pub fn from_char(ch: char) -> Self {
		Self {
			key: KeyCode::Char(ch),
			mods: KeyModifiers::NONE,
		}
	}

	pub fn new(key: KeyCode, mods: KeyModifiers) -> Self {
		Self { key, mods }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Action {
	Quit,
	CursorDown,
	CursorUp,
	CursorTop,
	CursorBottom,
	CursorToPlaying,
	CursorToPlayingLine,
	SeekRelative(f32),
	SeekBackwards,
	SeekForwards,
	SeekToCursor,
	SeekToCursorLine,
	TogglePause,
	VolumeDown,
	VolumeUp,
	VolumeDownSlightly,
	VolumeUpSlightly,
	SpeedDown,
	SpeedUp,
	SpeedReset,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyMap {
	map: HashMap<KeyChord, Action>,
}

impl KeyMap {
	pub fn get_action(&self, key_chord: KeyChord) -> Option<Action> {
		self.map.get(&key_chord).map(|&k| k)
	}
}

impl Default for KeyMap {
	fn default() -> Self {
		Self {
			map: HashMap::from([
				(KeyChord::from_char('q'), Action::Quit),
				(KeyChord::from_char('j'), Action::CursorDown),
				(KeyChord::from_char('k'), Action::CursorUp),
				(KeyChord::from_char('g'), Action::CursorTop),
				(KeyChord::from_char('G'), Action::CursorBottom),
				(KeyChord::from_char('t'), Action::CursorToPlaying),
				(KeyChord::from_char('T'), Action::CursorToPlayingLine),
				(KeyChord::from_char('0'), Action::SeekRelative(0.0)),
				(KeyChord::from_char('1'), Action::SeekRelative(0.1)),
				(KeyChord::from_char('2'), Action::SeekRelative(0.2)),
				(KeyChord::from_char('3'), Action::SeekRelative(0.3)),
				(KeyChord::from_char('4'), Action::SeekRelative(0.4)),
				(KeyChord::from_char('5'), Action::SeekRelative(0.5)),
				(KeyChord::from_char('6'), Action::SeekRelative(0.6)),
				(KeyChord::from_char('7'), Action::SeekRelative(0.7)),
				(KeyChord::from_char('8'), Action::SeekRelative(0.8)),
				(KeyChord::from_char('9'), Action::SeekRelative(0.9)),
				(KeyChord::from_char('H'), Action::SeekBackwards),
				(KeyChord::from_char('L'), Action::SeekForwards),
				(KeyChord::from_char('f'), Action::SeekToCursor),
				(KeyChord::from_char('F'), Action::SeekToCursorLine),
				(KeyChord::from_char('r'), Action::TogglePause),
				(KeyChord::from_char('['), Action::VolumeDown),
				(KeyChord::from_char(']'), Action::VolumeUp),
				(KeyChord::from_char('{'), Action::VolumeDownSlightly),
				(KeyChord::from_char('}'), Action::VolumeUpSlightly),
				(KeyChord::from_char('-'), Action::SpeedDown),
				(KeyChord::from_char('+'), Action::SpeedUp),
				(KeyChord::from_char('='), Action::SpeedReset),
			]),
		}
	}
}
