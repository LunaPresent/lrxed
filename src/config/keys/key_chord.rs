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
		todo!()
	}
}
