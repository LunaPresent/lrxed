use std::{collections::HashMap, fmt};

use crossterm::event::{KeyCode, KeyModifiers};
use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeMap};
use strum::{EnumCount, IntoEnumIterator};

use super::{Action, Context, KeyChord, action::KeyMapContextConfig};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMap {
	pub(super) map: [HashMap<KeyChord, Action>; Context::COUNT],
}

impl KeyMap {
	pub fn get_action(&self, key_chord: KeyChord, context: Context) -> Option<Action> {
		self.map[context as usize].get(&key_chord).map(|&k| k)
	}
}

impl Default for KeyMap {
	fn default() -> Self {
		let mut keymap = Self {
			map: Default::default(),
		};
		keymap.map[Context::Global as usize] = HashMap::from([
			(KeyChord::from_char('q'), Action::Quit),
			(
				KeyChord::new(KeyCode::Enter, KeyModifiers::NONE),
				Action::Confirm,
			),
			(
				KeyChord::new(KeyCode::Esc, KeyModifiers::NONE),
				Action::Cancel,
			),
			(KeyChord::from_char('j'), Action::MoveCursorY { amount: 1 }),
			(
				KeyChord::new(KeyCode::Down, KeyModifiers::NONE),
				Action::MoveCursorY { amount: 1 },
			),
			(KeyChord::from_char('k'), Action::MoveCursorY { amount: -1 }),
			(
				KeyChord::new(KeyCode::Up, KeyModifiers::NONE),
				Action::MoveCursorY { amount: -1 },
			),
			(KeyChord::from_char('h'), Action::MoveCursorX { amount: -1 }),
			(
				KeyChord::new(KeyCode::Left, KeyModifiers::NONE),
				Action::MoveCursorX { amount: -1 },
			),
			(KeyChord::from_char('l'), Action::MoveCursorX { amount: 1 }),
			(
				KeyChord::new(KeyCode::Right, KeyModifiers::NONE),
				Action::MoveCursorX { amount: 1 },
			),
			(KeyChord::from_char('g'), Action::SetCursorY { y: 0 }),
			(KeyChord::from_char('G'), Action::SetCursorY { y: u16::MAX }),
			(KeyChord::from_char('_'), Action::SetCursorX { x: 0 }),
			(
				KeyChord::new(KeyCode::Home, KeyModifiers::NONE),
				Action::SetCursorX { x: 0 },
			),
			(KeyChord::from_char('$'), Action::SetCursorX { x: u16::MAX }),
			(
				KeyChord::new(KeyCode::End, KeyModifiers::NONE),
				Action::SetCursorX { x: u16::MAX },
			),
		]);
		keymap.map[Context::Editor as usize] = HashMap::from([
			(
				KeyChord::new(KeyCode::Char('w'), KeyModifiers::CONTROL),
				Action::Save,
			),
			(KeyChord::from_char('t'), Action::CursorToPlaying),
			(KeyChord::from_char('T'), Action::CursorToPlayingLine),
			(
				KeyChord::from_char('0'),
				Action::SeekRelative { progress: 0.0 },
			),
			(
				KeyChord::from_char('1'),
				Action::SeekRelative { progress: 0.1 },
			),
			(
				KeyChord::from_char('2'),
				Action::SeekRelative { progress: 0.2 },
			),
			(
				KeyChord::from_char('3'),
				Action::SeekRelative { progress: 0.3 },
			),
			(
				KeyChord::from_char('4'),
				Action::SeekRelative { progress: 0.4 },
			),
			(
				KeyChord::from_char('5'),
				Action::SeekRelative { progress: 0.5 },
			),
			(
				KeyChord::from_char('6'),
				Action::SeekRelative { progress: 0.6 },
			),
			(
				KeyChord::from_char('7'),
				Action::SeekRelative { progress: 0.7 },
			),
			(
				KeyChord::from_char('8'),
				Action::SeekRelative { progress: 0.8 },
			),
			(
				KeyChord::from_char('9'),
				Action::SeekRelative { progress: 0.9 },
			),
			(
				KeyChord::from_char('H'),
				Action::SeekBackwards { seconds: 5. },
			),
			(
				KeyChord::from_char('L'),
				Action::SeekForwards { seconds: 5. },
			),
			(KeyChord::from_char('f'), Action::SeekToCursor),
			(KeyChord::from_char('F'), Action::SeekToCursorLine),
			(KeyChord::from_char('r'), Action::TogglePause),
			(
				KeyChord::from_char('['),
				Action::ChangeVolume { percentage: -10 },
			),
			(
				KeyChord::from_char(']'),
				Action::ChangeVolume { percentage: 10 },
			),
			(
				KeyChord::from_char('{'),
				Action::ChangeVolume { percentage: -1 },
			),
			(
				KeyChord::from_char('}'),
				Action::ChangeVolume { percentage: 1 },
			),
			(
				KeyChord::from_char('-'),
				Action::ChangeSpeed { percentage: -5 },
			),
			(
				KeyChord::from_char('+'),
				Action::ChangeSpeed { percentage: 5 },
			),
			(KeyChord::from_char('='), Action::ResetSpeed),
			(KeyChord::from_char('u'), Action::Undo),
			(
				KeyChord::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
				Action::Redo,
			),
			(KeyChord::from_char(' '), Action::SyncTimestamp),
			(
				KeyChord::from_char('s'),
				Action::AdjustTimestamp { centis: 100 },
			),
			(
				KeyChord::from_char('S'),
				Action::AdjustTimestamp { centis: -100 },
			),
			(
				KeyChord::from_char('d'),
				Action::AdjustTimestamp { centis: 10 },
			),
			(
				KeyChord::from_char('D'),
				Action::AdjustTimestamp { centis: -10 },
			),
			(
				KeyChord::from_char('c'),
				Action::AdjustTimestamp { centis: 1 },
			),
			(
				KeyChord::from_char('C'),
				Action::AdjustTimestamp { centis: -1 },
			),
			(KeyChord::from_char('I'), Action::OpenInEditor),
		]);
		keymap.map[Context::ConfirmBox as usize] = HashMap::from([
			(KeyChord::from_char('y'), Action::Yes),
			(KeyChord::from_char('n'), Action::No),
			(KeyChord::from_char('c'), Action::Cancel),
		]);
		keymap.map[Context::FileBrowser as usize] = HashMap::from([(
			KeyChord::new(KeyCode::Enter, KeyModifiers::NONE),
			Action::OpenInEditor,
		)]);
		keymap
	}
}

impl Serialize for KeyMap {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut map = serializer.serialize_map(Some(self.map.len()))?;
		for (context, key_map_context) in Context::iter().zip(self.map.iter()) {
			map.serialize_entry(&context, &KeyMapContextConfig::new(key_map_context))?;
		}
		map.end()
	}
}

struct KeyMapVisitor;

impl<'de> Visitor<'de> for KeyMapVisitor {
	type Value = KeyMap;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter
			.write_str("a map with Context as key and another map from KeyChord to Action as value")
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: serde::de::MapAccess<'de>,
	{
		let mut key_map = Self::Value::default();
		while let Some((context, config)) = access.next_entry::<Context, KeyMapContextConfig>()? {
			let key_map_context = &mut key_map.map[context as usize];
			for action_config in config.values() {
				key_map_context
					.retain(|&k, &mut a| k != action_config.key() && a != action_config.action());
			}
			for action_config in config.values() {
				key_map_context.insert(action_config.key(), action_config.action());
			}
		}
		Ok(key_map)
	}
}

impl<'de> Deserialize<'de> for KeyMap {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_map(KeyMapVisitor)
	}
}
