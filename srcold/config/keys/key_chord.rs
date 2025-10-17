use std::{
	fmt::{self, Display},
	str::FromStr,
};

use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyModifiers};
use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay, DeserializeFromStr)]
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

impl Display for KeyChord {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.mods.contains(KeyModifiers::SHIFT) {
			write!(f, "S-")?;
		}
		if self.mods.contains(KeyModifiers::CONTROL) {
			write!(f, "C-")?;
		}
		if self.mods.contains(KeyModifiers::ALT) {
			write!(f, "A-")?;
		}
		if self.mods.contains(KeyModifiers::SUPER) {
			write!(f, "D-")?;
		}
		write!(f, "{}", self.key)
	}
}

impl FromStr for KeyChord {
	type Err = eyre::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().count() {
			0 => Err(eyre::eyre!("Invalid key chord")),
			1 => Ok(KeyChord::from_char(s.chars().next().unwrap())),
			_ => {
				let mut mods = KeyModifiers::NONE;
				let split_i = s[..s.len() - 1]
					.rfind('-')
					.map(|i| i + '-'.len_utf8())
					.unwrap_or(0);
				let (mod_str, key_str) = s.split_at(split_i);
				for m in mod_str.split_terminator('-') {
					mods.insert(match m {
						"S" | "s" => KeyModifiers::SHIFT,
						"C" | "c" => KeyModifiers::CONTROL,
						"A" | "a" | "M" | "m" => KeyModifiers::ALT,
						"D" | "d" => KeyModifiers::SUPER,
						_ => return Err(eyre::eyre!("Invalid modifier identifier")),
					});
				}
				let key = if key_str.chars().count() == 1 {
					KeyCode::Char(key_str.chars().next().unwrap())
				} else {
					match key_str.to_lowercase().as_str() {
						"backspace" => KeyCode::Backspace,
						"enter" => KeyCode::Enter,
						"left" => KeyCode::Left,
						"right" => KeyCode::Right,
						"up" => KeyCode::Up,
						"down" => KeyCode::Down,
						"home" => KeyCode::Home,
						"end" => KeyCode::End,
						"pageup" => KeyCode::PageUp,
						"pagedown" => KeyCode::PageDown,
						"tab" => KeyCode::Tab,
						"backtab" => KeyCode::BackTab,
						"delete" => KeyCode::Delete,
						"insert" => KeyCode::Insert,
						"null" => KeyCode::Null,
						"esc" => KeyCode::Esc,
						"capslock" => KeyCode::CapsLock,
						"scrolllock" => KeyCode::ScrollLock,
						"numlock" => KeyCode::NumLock,
						"printscreen" => KeyCode::PrintScreen,
						"pause" => KeyCode::Pause,
						"menu" => KeyCode::Menu,
						"keypadbegin" => KeyCode::KeypadBegin,
						_ => return Err(eyre::eyre!("Invalid key identifier")),
					}
				};
				Ok(KeyChord::new(key, mods))
			}
		}
	}
}
