use core::fmt;
use core::str::FromStr;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::KeyChordParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct KeyChord {
	pub key: KeyCode,
	pub mods: KeyModifiers,
}

#[allow(
	clippy::derive_ord_xor_partial_ord,
	reason = "crossterm mistakenly only implements PartialOrd and not Ord"
)]
impl Ord for KeyChord {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.partial_cmp(other).expect(
			"KeyCode and KeyModifiers could derive Ord, they just forgot. \
				Therefore partial_cmp should never return None.",
		)
	}
}

impl KeyChord {
	pub fn new(key: KeyCode, mods: KeyModifiers) -> Self {
		Self { key, mods }.normalise()
	}

	pub fn from_char(ch: char) -> Self {
		Self::new(KeyCode::Char(ch), KeyModifiers::NONE)
	}

	pub fn from_event(event: KeyEvent) -> Self {
		Self::new(event.code, event.modifiers)
	}

	pub fn normalise(self) -> Self {
		if let KeyCode::Char(c) = self.key
			&& c.is_ascii_uppercase()
		{
			Self {
				key: self.key,
				mods: self.mods | KeyModifiers::SHIFT,
			}
		} else if let KeyCode::Char(c) = self.key
			&& self.mods.contains(KeyModifiers::SHIFT)
		{
			Self {
				key: KeyCode::Char(c.to_ascii_uppercase()),
				mods: self.mods,
			}
		} else {
			self
		}
	}
}

impl fmt::Display for KeyChord {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.mods.contains(KeyModifiers::SUPER) {
			write!(f, "D-")?;
		}
		if self.mods.contains(KeyModifiers::ALT) {
			write!(f, "A-")?;
		}
		if self.mods.contains(KeyModifiers::CONTROL) {
			write!(f, "C-")?;
		}
		if self.mods.contains(KeyModifiers::SHIFT) {
			write!(f, "S-")?;
		}
		match self.key {
			KeyCode::Null => write!(f, "Nul"),
			KeyCode::Backspace => write!(f, "BS"),
			KeyCode::Tab => write!(f, "Tab"),
			KeyCode::Enter => write!(f, "CR"),
			KeyCode::Esc => write!(f, "Esc"),
			KeyCode::Char(' ') => write!(f, "Space"),
			KeyCode::Char('<') => write!(f, "lt"),
			KeyCode::Char('\\') => write!(f, "Bslash"),
			KeyCode::Char('|') => write!(f, "Bar"),
			KeyCode::Delete => write!(f, "Del"),
			KeyCode::Up => write!(f, "Up"),
			KeyCode::Down => write!(f, "Down"),
			KeyCode::Left => write!(f, "Left"),
			KeyCode::Right => write!(f, "Right"),
			KeyCode::Home => write!(f, "Home"),
			KeyCode::End => write!(f, "End"),
			KeyCode::PageUp => write!(f, "PageUp"),
			KeyCode::PageDown => write!(f, "PageDown"),
			KeyCode::Insert => write!(f, "Insert"),
			KeyCode::Char(c) => write!(f, "{}", c),
			KeyCode::F(n) => write!(f, "F{}", n),
			key => write!(f, "{}", key),
		}
	}
}

impl FromStr for KeyChord {
	type Err = KeyChordParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.chars().count() {
			0 => Err(KeyChordParseError::EmptyString),
			1 => Ok(KeyChord::from_char(s.chars().next().unwrap())),
			_ => {
				let mut mods = KeyModifiers::NONE;
				let split_i = s[..s.len() - 1]
					.rfind('-')
					.map(|i| i + '-'.len_utf8())
					.unwrap_or(0);
				let (mod_str, key_str) = s.split_at(split_i);
				for m in mod_str.split_terminator('-') {
					let new_mod = match m {
						"S" | "s" => KeyModifiers::SHIFT,
						"C" | "c" => KeyModifiers::CONTROL,
						"A" | "a" | "M" | "m" => KeyModifiers::ALT,
						"D" | "d" => KeyModifiers::SUPER,
						_ => return Err(KeyChordParseError::InvalidModifier),
					};
					if mods.contains(new_mod) {
						return Err(KeyChordParseError::DuplicateModifier);
					}
					mods |= new_mod;
				}
				let key = if key_str.chars().count() == 1 {
					KeyCode::Char(key_str.chars().next().unwrap())
				} else {
					match key_str.to_lowercase().as_str() {
						"nul" => KeyCode::Null,
						"bs" => KeyCode::Backspace,
						"tab" => KeyCode::Tab,
						"cr" | "return" | "enter" | "eol" => KeyCode::Enter,
						"esc" => KeyCode::Esc,
						"space" => KeyCode::Char(' '),
						"lt" => KeyCode::Char('<'),
						"bslash" => KeyCode::Char('\\'),
						"bar" => KeyCode::Char('|'),
						"del" => KeyCode::Delete,
						"up" => KeyCode::Up,
						"down" => KeyCode::Down,
						"left" => KeyCode::Left,
						"right" => KeyCode::Right,
						"home" => KeyCode::Home,
						"end" => KeyCode::End,
						"pageup" => KeyCode::PageUp,
						"pagedown" => KeyCode::PageDown,
						"insert" => KeyCode::Insert,
						"f1" => KeyCode::F(1),
						"f2" => KeyCode::F(2),
						"f3" => KeyCode::F(3),
						"f4" => KeyCode::F(4),
						"f5" => KeyCode::F(5),
						"f6" => KeyCode::F(6),
						"f7" => KeyCode::F(7),
						"f8" => KeyCode::F(8),
						"f9" => KeyCode::F(9),
						"f10" => KeyCode::F(10),
						"f11" => KeyCode::F(11),
						"f12" => KeyCode::F(12),
						_ => return Err(KeyChordParseError::InvalidKey),
					}
				};
				Ok(KeyChord::new(key, mods))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new_does_normalise() {
		let lower_shift = KeyChord {
			key: KeyCode::Char('a'),
			mods: KeyModifiers::SHIFT,
		};
		let new = KeyChord::new(KeyCode::Char('A'), KeyModifiers::NONE);

		assert_eq!(new, lower_shift.normalise());
	}

	#[test]
	fn normalise_retains_mods() {
		let with_ctrl_no_caps = KeyChord {
			key: KeyCode::Char('a'),
			mods: KeyModifiers::CONTROL,
		}
		.normalise();
		let with_ctrl = KeyChord {
			key: KeyCode::Char('A'),
			mods: KeyModifiers::CONTROL,
		}
		.normalise();

		assert_eq!(with_ctrl_no_caps.mods, KeyModifiers::CONTROL);
		assert_eq!(with_ctrl.mods, KeyModifiers::CONTROL | KeyModifiers::SHIFT)
	}

	#[test]
	fn normalise_equivalent_makes_equal() {
		let lower_no_shift = KeyChord {
			key: KeyCode::Char('a'),
			mods: KeyModifiers::NONE,
		};
		let upper_no_shift = KeyChord {
			key: KeyCode::Char('A'),
			mods: KeyModifiers::NONE,
		};
		let lower_shift = KeyChord {
			key: KeyCode::Char('a'),
			mods: KeyModifiers::SHIFT,
		};
		let upper_shift = KeyChord {
			key: KeyCode::Char('A'),
			mods: KeyModifiers::SHIFT,
		};

		assert_ne!(lower_no_shift, upper_no_shift);
		assert_ne!(lower_no_shift, lower_shift);
		assert_ne!(lower_no_shift, upper_shift);
		assert_ne!(upper_no_shift, lower_shift);
		assert_ne!(upper_no_shift, upper_shift);
		assert_ne!(lower_shift, upper_shift);

		let lower_no_shift = lower_no_shift.normalise();
		let upper_no_shift = upper_no_shift.normalise();
		let lower_shift = lower_shift.normalise();
		let upper_shift = upper_shift.normalise();

		assert_ne!(lower_no_shift, upper_no_shift);
		assert_ne!(lower_no_shift, lower_shift);
		assert_ne!(lower_no_shift, upper_shift);

		assert_eq!(upper_no_shift, lower_shift);
		assert_eq!(upper_no_shift, upper_shift);
		assert_eq!(lower_shift, upper_shift);
	}

	#[test]
	fn display() {
		let f = KeyChord {
			key: KeyCode::Char('f'),
			mods: KeyModifiers::NONE,
		};
		let alt_a = KeyChord {
			key: KeyCode::Char('a'),
			mods: KeyModifiers::ALT,
		};
		let hyper_space = KeyChord {
			key: KeyCode::Char(' '),
			mods: KeyModifiers::SUPER
				| KeyModifiers::ALT
				| KeyModifiers::CONTROL
				| KeyModifiers::SHIFT,
		};
		let esc = KeyChord {
			key: KeyCode::Esc,
			mods: KeyModifiers::NONE,
		};
		let shift_f10 = KeyChord {
			key: KeyCode::F(10),
			mods: KeyModifiers::SHIFT,
		};

		assert_eq!(f.to_string(), "f");
		assert_eq!(alt_a.to_string(), "A-a");
		assert_eq!(hyper_space.to_string(), "D-A-C-S-Space");
		assert_eq!(esc.to_string(), "Esc");
		assert_eq!(shift_f10.to_string(), "S-F10");
	}

	#[test]
	fn from_str_happy_flow() -> Result<(), KeyChordParseError> {
		let f = KeyChord {
			key: KeyCode::Char('f'),
			mods: KeyModifiers::NONE,
		};
		let alt_a = KeyChord {
			key: KeyCode::Char('a'),
			mods: KeyModifiers::ALT,
		};
		let hyper_space = KeyChord {
			key: KeyCode::Char(' '),
			mods: KeyModifiers::SUPER
				| KeyModifiers::ALT
				| KeyModifiers::CONTROL
				| KeyModifiers::SHIFT,
		};
		let esc = KeyChord {
			key: KeyCode::Esc,
			mods: KeyModifiers::NONE,
		};
		let shift_f10 = KeyChord {
			key: KeyCode::F(10),
			mods: KeyModifiers::SHIFT,
		};

		assert_eq!("f".parse::<KeyChord>()?, f);
		assert_eq!("A-a".parse::<KeyChord>()?, alt_a);
		assert_eq!("D-A-C-S-Space".parse::<KeyChord>()?, hyper_space);
		assert_eq!("a-C-d-S-sPaCe".parse::<KeyChord>()?, hyper_space);
		assert_eq!("Esc".parse::<KeyChord>()?, esc);
		assert_eq!("s-f10".parse::<KeyChord>()?, shift_f10);

		Ok(())
	}

	#[test]
	fn from_str_invalid_input_errors() {
		assert_eq!("".parse::<KeyChord>(), Err(KeyChordParseError::EmptyString));
		assert_eq!(
			"Z-f".parse::<KeyChord>(),
			Err(KeyChordParseError::InvalidModifier)
		);
		assert_eq!(
			"CC-f".parse::<KeyChord>(),
			Err(KeyChordParseError::InvalidModifier)
		);
		assert_eq!(
			"C-c-f".parse::<KeyChord>(),
			Err(KeyChordParseError::DuplicateModifier)
		);
		assert_eq!(
			"foo".parse::<KeyChord>(),
			Err(KeyChordParseError::InvalidKey)
		);
		assert_eq!(
			"F99".parse::<KeyChord>(),
			Err(KeyChordParseError::InvalidKey)
		);
	}
}
