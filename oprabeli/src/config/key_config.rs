use core::fmt;
use core::marker::PhantomData;

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap as _;
use serde::{Deserialize, Serialize, Serializer};

use super::{ConvertAction, KeyMap, KeyMapping, KeySequence, TryConvertAction, util::OneOrMany};

#[derive(Debug)]
struct InputMapping<A> {
	action: A,
	inputs: OneOrMany<KeySequence>,
}

// TODO: documentation
#[derive(Debug)]
pub struct KeyConfig<A>(Vec<InputMapping<A>>);

impl<A> Default for KeyConfig<A> {
	fn default() -> Self {
		Self(Vec::new())
	}
}

impl<A> KeyConfig<A>
where
	A: ConvertAction,
{
	// TODO: documentation
	pub fn generate_key_map(&self) -> KeyMap<A::Event> {
		let mut vec = Vec::with_capacity(
			self.0
				.iter()
				.map(|input_mapping| input_mapping.inputs.len())
				.sum(),
		);

		for input_mapping in &self.0 {
			for input in &input_mapping.inputs {
				vec.push(KeyMapping::new(
					input.clone(),
					input_mapping.action.to_event(),
				));
			}
		}

		KeyMap::from(vec)
	}
}

impl<A> KeyConfig<A>
where
	A: TryConvertAction,
{
	// TODO: documentation
	pub fn try_generate_key_map(&self) -> Result<KeyMap<A::Event>, A::Error> {
		let mut vec = Vec::with_capacity(
			self.0
				.iter()
				.map(|input_mapping| input_mapping.inputs.len())
				.sum(),
		);

		for input_mapping in &self.0 {
			for input in &input_mapping.inputs {
				vec.push(KeyMapping::new(
					input.clone(),
					input_mapping.action.try_to_event()?,
				));
			}
		}

		Ok(KeyMap::from(vec))
	}
}

impl<A> Serialize for KeyConfig<A>
where
	A: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(self.0.len()))?;
		for input_mapping in &self.0 {
			map.serialize_entry(&input_mapping.action, &input_mapping.inputs)?;
		}
		map.end()
	}
}

#[derive(Debug)]
struct InputMapVisitor<A>(PhantomData<A>);

impl<'de, A> Visitor<'de> for InputMapVisitor<A>
where
	A: Deserialize<'de>,
{
	type Value = KeyConfig<A>;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter
			.write_str("a map with Context as key and another map from KeyChord to Action as value")
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut input_mappings = Vec::new();
		while let Some((action, inputs)) = access.next_entry::<A, OneOrMany<KeySequence>>()? {
			input_mappings.push(InputMapping { action, inputs });
		}
		Ok(KeyConfig(input_mappings))
	}
}

impl<'de, A> Deserialize<'de> for KeyConfig<A>
where
	A: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_map(InputMapVisitor(PhantomData))
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr as _;

	use serde_json::json;

	use super::*;

	#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
	#[serde(rename_all = "kebab-case")]
	pub enum InputAction {
		Quit,
		CursorUp,
		CursorDown,
		CursorLeft,
		CursorRight,
	}

	#[derive(Debug, PartialEq, Eq)]
	pub enum AppEvent {
		Quit,
		MoveCursor(Direction),
	}

	#[derive(Debug, PartialEq, Eq)]
	pub enum Direction {
		Up,
		Down,
		Left,
		Right,
	}

	impl ConvertAction for InputAction {
		type Event = AppEvent;

		fn to_event(&self) -> Self::Event {
			match *self {
				InputAction::Quit => AppEvent::Quit,
				InputAction::CursorUp => AppEvent::MoveCursor(Direction::Up),
				InputAction::CursorDown => AppEvent::MoveCursor(Direction::Down),
				InputAction::CursorLeft => AppEvent::MoveCursor(Direction::Left),
				InputAction::CursorRight => AppEvent::MoveCursor(Direction::Right),
			}
		}
	}

	#[test]
	fn generate_key_map() -> eyre::Result<()> {
		let key_config = KeyConfig(vec![
			InputMapping {
				action: InputAction::Quit,
				inputs: OneOrMany::One(KeySequence::from_str("q")?),
			},
			InputMapping {
				action: InputAction::CursorUp,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("k")?,
					KeySequence::from_str("<Up>")?,
				]),
			},
			InputMapping {
				action: InputAction::CursorDown,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("j")?,
					KeySequence::from_str("<Down>")?,
				]),
			},
			InputMapping {
				action: InputAction::CursorLeft,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("h")?,
					KeySequence::from_str("<Left>")?,
				]),
			},
			InputMapping {
				action: InputAction::CursorRight,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("l")?,
					KeySequence::from_str("<Right>")?,
				]),
			},
		]);

		let key_map = key_config.generate_key_map();

		assert_eq!(key_map.len(), 9);

		assert_eq!(key_map[0].key_sequence.to_string(), "<Left>");
		assert_eq!(key_map[0].app_event, AppEvent::MoveCursor(Direction::Left));

		assert_eq!(key_map[1].key_sequence.to_string(), "<Right>");
		assert_eq!(key_map[1].app_event, AppEvent::MoveCursor(Direction::Right));

		assert_eq!(key_map[2].key_sequence.to_string(), "<Up>");
		assert_eq!(key_map[2].app_event, AppEvent::MoveCursor(Direction::Up));

		assert_eq!(key_map[3].key_sequence.to_string(), "<Down>");
		assert_eq!(key_map[3].app_event, AppEvent::MoveCursor(Direction::Down));

		assert_eq!(key_map[4].key_sequence.to_string(), "h");
		assert_eq!(key_map[4].app_event, AppEvent::MoveCursor(Direction::Left));

		assert_eq!(key_map[5].key_sequence.to_string(), "j");
		assert_eq!(key_map[5].app_event, AppEvent::MoveCursor(Direction::Down));

		assert_eq!(key_map[6].key_sequence.to_string(), "k");
		assert_eq!(key_map[6].app_event, AppEvent::MoveCursor(Direction::Up));

		assert_eq!(key_map[7].key_sequence.to_string(), "l");
		assert_eq!(key_map[7].app_event, AppEvent::MoveCursor(Direction::Right));

		assert_eq!(key_map[8].key_sequence.to_string(), "q");
		assert_eq!(key_map[8].app_event, AppEvent::Quit);

		Ok(())
	}

	#[test]
	fn serialise() -> eyre::Result<()> {
		let key_config = KeyConfig(vec![
			InputMapping {
				action: InputAction::Quit,
				inputs: OneOrMany::One(KeySequence::from_str("q")?),
			},
			InputMapping {
				action: InputAction::CursorUp,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("k")?,
					KeySequence::from_str("<Up>")?,
				]),
			},
			InputMapping {
				action: InputAction::CursorDown,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("j")?,
					KeySequence::from_str("<Down>")?,
				]),
			},
			InputMapping {
				action: InputAction::CursorLeft,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("h")?,
					KeySequence::from_str("<Left>")?,
				]),
			},
			InputMapping {
				action: InputAction::CursorRight,
				inputs: OneOrMany::Many(vec![
					KeySequence::from_str("l")?,
					KeySequence::from_str("<Right>")?,
				]),
			},
		]);

		let to_json = serde_json::to_value(&key_config)?;

		let json = json!({
			"quit": "q",
			"cursor-up": ["k", "<Up>"],
			"cursor-down": ["j", "<Down>"],
			"cursor-left": ["h", "<Left>"],
			"cursor-right": ["l", "<Right>"]
		});

		assert_eq!(to_json, json);

		Ok(())
	}

	#[test]
	fn deserialise() -> eyre::Result<()> {
		let json = json!({
			"quit": "q",
			"cursor-up": ["k", "<Up>"],
			"cursor-down": ["j", "<Down>"],
			"cursor-left": ["h", "<Left>"],
			"cursor-right": ["l", "<Right>"]
		});

		let key_config: KeyConfig<InputAction> = serde_json::from_value(json)?;

		println!("{}", serde_json::to_string_pretty(&key_config)?);

		assert_eq!(key_config.0.len(), 5);
		assert_eq!(key_config.0[0].action, InputAction::CursorDown);
		assert_eq!(key_config.0[1].action, InputAction::CursorLeft);
		assert_eq!(key_config.0[2].action, InputAction::CursorRight);
		assert_eq!(key_config.0[3].action, InputAction::CursorUp);
		assert_eq!(key_config.0[4].action, InputAction::Quit);

		Ok(())
	}
}
