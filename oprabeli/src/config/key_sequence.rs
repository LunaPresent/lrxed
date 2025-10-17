use core::fmt;
use core::ops::Deref;
use core::str::FromStr;

use serde_with::{DeserializeFromStr, SerializeDisplay};
use smallvec::SmallVec;

use super::{KeyChord, KeyChordParseError};

#[derive(
	Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, SerializeDisplay, DeserializeFromStr,
)]
pub struct KeySequence(SmallVec<[KeyChord; 4]>);

impl KeySequence {
	pub fn from_vec(vec: Vec<KeyChord>) -> Self {
		Self(SmallVec::from_vec(vec))
	}

	pub fn from_slice(slice: &[KeyChord]) -> Self {
		Self(SmallVec::from_slice(slice))
	}
}

impl FromIterator<KeyChord> for KeySequence {
	fn from_iter<I: IntoIterator<Item = KeyChord>>(iter: I) -> KeySequence {
		Self(SmallVec::from_iter(iter))
	}
}

impl fmt::Display for KeySequence {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for key_chord in &self.0 {
			let string = key_chord.to_string();
			if string.chars().count() == 1 {
				write!(f, "{}", string)?;
			} else {
				write!(f, "<{}>", string)?;
			}
		}
		Ok(())
	}
}

impl FromStr for KeySequence {
	type Err = KeyChordParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut key_chords = SmallVec::default();
		let mut in_tag = false;
		let mut chord_start = 0;

		for (idx, c) in s.char_indices() {
			if in_tag {
				if c == '>' {
					in_tag = false;
					key_chords.push(s[chord_start..idx].parse()?);
				}
			} else if c == '<' {
				in_tag = true;
				chord_start = idx + 1;
			} else {
				key_chords.push(s[idx..idx + 1].parse()?);
			}
		}

		if in_tag {
			Err(KeyChordParseError::UnclosedTag)
		} else {
			Ok(KeySequence(key_chords))
		}
	}
}

impl Deref for KeySequence {
	type Target = [KeyChord];

	fn deref(&self) -> &Self::Target {
		self.0.as_slice()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crossterm::event::{KeyCode, KeyModifiers};
	use smallvec::smallvec;

	#[test]
	fn display() {
		let f = KeySequence(smallvec![KeyChord::from_char('f')]);
		let esc = KeySequence(smallvec![KeyChord::new(KeyCode::Esc, KeyModifiers::NONE)]);
		let e_s_c = KeySequence(smallvec![
			KeyChord::from_char('E'),
			KeyChord::from_char('s'),
			KeyChord::from_char('c'),
		]);
		let alt_a = KeySequence(smallvec![KeyChord::new(
			KeyCode::Char('a'),
			KeyModifiers::ALT,
		)]);
		let f_alt_a = KeySequence(smallvec![
			KeyChord::from_char('f'),
			KeyChord::new(KeyCode::Char('a'), KeyModifiers::ALT),
		]);
		let alt_a_f = KeySequence(smallvec![
			KeyChord::new(KeyCode::Char('a'), KeyModifiers::ALT),
			KeyChord::from_char('f'),
		]);
		let space_ctrl_w_left_h_esc = KeySequence(smallvec![
			KeyChord::from_char(' '),
			KeyChord::new(KeyCode::Char('w'), KeyModifiers::CONTROL),
			KeyChord::new(KeyCode::Left, KeyModifiers::NONE),
			KeyChord::from_char('h'),
			KeyChord::new(KeyCode::Esc, KeyModifiers::NONE),
		]);

		assert_eq!(f.to_string(), "f");
		assert_eq!(esc.to_string(), "<Esc>");
		assert_eq!(e_s_c.to_string(), "<S-E>sc");
		assert_eq!(alt_a.to_string(), "<A-a>");
		assert_eq!(f_alt_a.to_string(), "f<A-a>");
		assert_eq!(alt_a_f.to_string(), "<A-a>f");
		assert_eq!(
			space_ctrl_w_left_h_esc.to_string(),
			"<Space><C-w><Left>h<Esc>"
		);
	}

	#[test]
	fn from_str_happy_flow() -> Result<(), KeyChordParseError> {
		let f = KeySequence(smallvec![KeyChord::from_char('f')]);
		let esc = KeySequence(smallvec![KeyChord::new(KeyCode::Esc, KeyModifiers::NONE)]);
		let e_s_c = KeySequence(smallvec![
			KeyChord::from_char('E'),
			KeyChord::from_char('s'),
			KeyChord::from_char('c'),
		]);
		let alt_a = KeySequence(smallvec![KeyChord::new(
			KeyCode::Char('a'),
			KeyModifiers::ALT,
		)]);
		let f_alt_a = KeySequence(smallvec![
			KeyChord::from_char('f'),
			KeyChord::new(KeyCode::Char('a'), KeyModifiers::ALT),
		]);
		let alt_a_f = KeySequence(smallvec![
			KeyChord::new(KeyCode::Char('a'), KeyModifiers::ALT),
			KeyChord::from_char('f'),
		]);
		let space_ctrl_w_left_h_esc = KeySequence(smallvec![
			KeyChord::from_char(' '),
			KeyChord::new(KeyCode::Char('w'), KeyModifiers::CONTROL),
			KeyChord::new(KeyCode::Left, KeyModifiers::NONE),
			KeyChord::from_char('h'),
			KeyChord::new(KeyCode::Esc, KeyModifiers::NONE),
		]);

		assert_eq!("f".parse::<KeySequence>()?, f);
		assert_eq!("<esc>".parse::<KeySequence>()?, esc);
		assert_eq!("Esc".parse::<KeySequence>()?, e_s_c);
		assert_eq!("<A-a>".parse::<KeySequence>()?, alt_a);
		assert_eq!("f<a-a>".parse::<KeySequence>()?, f_alt_a);
		assert_eq!("<a-a>f".parse::<KeySequence>()?, alt_a_f);
		assert_eq!(
			"<space><C-w><left>h<ESC>".parse::<KeySequence>()?,
			space_ctrl_w_left_h_esc
		);

		Ok(())
	}

	#[test]
	fn from_str_invalid_input_errors() {
		assert_eq!(
			"a<>b".parse::<KeySequence>(),
			Err(KeyChordParseError::EmptyString)
		);
		assert_eq!(
			"<C-r><Z-f>".parse::<KeySequence>(),
			Err(KeyChordParseError::InvalidModifier)
		);
		assert_eq!(
			"<CC-f>".parse::<KeySequence>(),
			Err(KeyChordParseError::InvalidModifier)
		);
		assert_eq!(
			"<C-c-f>".parse::<KeySequence>(),
			Err(KeyChordParseError::DuplicateModifier)
		);
		assert_eq!(
			"<Cw>".parse::<KeySequence>(),
			Err(KeyChordParseError::InvalidKey)
		);
		assert_eq!(
			"<C-w".parse::<KeySequence>(),
			Err(KeyChordParseError::UnclosedTag)
		);
	}
}
