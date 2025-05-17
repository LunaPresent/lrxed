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
	MoveCursorY(i16),
	MoveCursorX(i16),
	SetCursorY(u16),
	SetCursorX(u16),
	CursorToPlaying,
	CursorToPlayingLine,
	SeekRelative(f32),
	SeekBackwards,
	SeekForwards,
	SeekToCursor,
	SeekToCursorLine,
	TogglePause,
	ChangeVolume(i16),
	ChangeSpeed(i16),
	ResetSpeed,
	Undo,
	Redo,
	SyncTimestamp,
	AdjustTimestamp(i32),
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
				(KeyChord::from_char('j'), Action::MoveCursorY(1)),
				(KeyChord::from_char('k'), Action::MoveCursorY(-1)),
				(KeyChord::from_char('h'), Action::MoveCursorX(-1)),
				(KeyChord::from_char('l'), Action::MoveCursorX(1)),
				(KeyChord::from_char('g'), Action::SetCursorY(0)),
				(KeyChord::from_char('G'), Action::SetCursorY(u16::MAX)),
				(KeyChord::from_char('_'), Action::SetCursorX(0)),
				(KeyChord::from_char('$'), Action::SetCursorX(u16::MAX)),
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
				(KeyChord::from_char('['), Action::ChangeVolume(-10)),
				(KeyChord::from_char(']'), Action::ChangeVolume(10)),
				(KeyChord::from_char('{'), Action::ChangeVolume(-1)),
				(KeyChord::from_char('}'), Action::ChangeVolume(1)),
				(KeyChord::from_char('-'), Action::ChangeSpeed(-5)),
				(KeyChord::from_char('+'), Action::ChangeSpeed(5)),
				(KeyChord::from_char('='), Action::ResetSpeed),
				(KeyChord::from_char('u'), Action::Undo),
				(
					KeyChord::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
					Action::Redo,
				),
				(KeyChord::from_char(' '), Action::SyncTimestamp),
				(KeyChord::from_char('s'), Action::AdjustTimestamp(100)),
				(KeyChord::from_char('S'), Action::AdjustTimestamp(-100)),
				(KeyChord::from_char('d'), Action::AdjustTimestamp(10)),
				(KeyChord::from_char('D'), Action::AdjustTimestamp(-10)),
				(KeyChord::from_char('c'), Action::AdjustTimestamp(1)),
				(KeyChord::from_char('C'), Action::AdjustTimestamp(-1)),
			]),
		}
	}
}
