#![allow(dead_code)]
use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeMap};
use strum::{
	EnumCount, EnumDiscriminants, EnumIter, FromRepr, IntoDiscriminant, IntoEnumIterator,
	VariantNames,
};

use super::KeyChord;

macro_rules! define_actions {// {{{
	($($name:ident $({ $value:ident: $data:ty })?),+ $(,)?) => {

		#[derive(Debug, Clone, Copy, PartialEq, EnumDiscriminants, FromRepr)]
		#[strum_discriminants(name(ActionType))]
		#[strum_discriminants(derive(EnumCount, EnumIter, VariantNames, Serialize, Deserialize))]
		#[strum_discriminants(strum(serialize_all = "kebab-case"))]
		#[strum_discriminants(serde(rename_all = "kebab-case"))]
		pub enum Action {
			$($name$({ $value: $data })?),+
		}

		trait ConstActionType {
			const ACTION_TYPE: ActionType;
		}

		mod data {
			use serde::{Serialize, Deserialize};

			$(
			#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
			pub struct $name { pub key: super::KeyChord, $(pub $value: $data,)? }

			impl From<$name> for super::ActionConfig {
				fn from(inner: $name) -> Self {
					Self::$name(inner)
				}
			}

			impl super::ConstActionType for $name {
				const ACTION_TYPE: super::ActionType = super::ActionType::$name;
			}
			)+
		}

		#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
		#[serde(untagged)]
		pub enum ActionConfig {
			$($name (data::$name)),+
		}

		impl ActionConfig {
			pub fn new(action: Action, key: KeyChord) -> Self {
				match action {
					$(Action::$name$({ $value })? => Self::$name(data::$name{ $($value,)? key })),+
				}
			}

			pub fn new_default_inner(action_type: ActionType, key: KeyChord) -> Self {
				match Action::from_repr(action_type as usize).unwrap() {
					$(Action::$name$({ $value })? => Self::$name(data::$name{ $($value,)? key })),+
				}
			}

			pub fn action(&self) -> Action {
				match self {
					$(Self::$name(data::$name{ $($value,)? key: _ }) => {
						Action::$name$({ $value: *$value})?
					}),+
				}
			}

			pub fn key(&self) -> KeyChord {
				match self {
					$(Self::$name(data::$name{ $($value: _,)? key }) => *key),+
				}
			}
		}

		struct KeyMapContextConfigVisitor;

		impl<'de> Visitor<'de> for KeyMapContextConfigVisitor {
			type Value = KeyMapContextConfig;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str(
					"a map with ActionType as key and ActionConfig or list of ActionConfigs as value",
				)
			}

			fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
			where
				M: serde::de::MapAccess<'de>,
			{
				let mut key_map = Self::Value::default();
				while let Some(action_type) = access.next_key::<ActionType>()? {
					match action_type {
						$(ActionType::$name => {
							let config_or_list =
								access.next_value::<ActionConfigOrList<data::$name>>()?;
							key_map.map[action_type as usize] = config_or_list.into();
						}),+
					}
				}
				Ok(key_map)
			}
		}
	};
} // }}}

define_actions! {
	NoOp,
	Quit,
	Confirm,
	Cancel,
	Yes,
	No,
	Save,
	MoveCursorY { amount: i16 },
	MoveCursorX { amount: i16 },
	SetCursorY { y: u16 },
	SetCursorX { x: u16 },
	CursorToPlaying,
	CursorToPlayingLine,
	SeekRelative { progress: f32 },
	SeekBackwards { seconds: f32 },
	SeekForwards { seconds: f32 },
	SeekToCursor,
	SeekToCursorLine,
	TogglePause,
	ChangeVolume { percentage: i16 },
	ChangeSpeed { percentage: i16 },
	ResetSpeed,
	Undo,
	Redo,
	SyncTimestamp,
	AdjustTimestamp { centis: i32 },
	OpenInEditor,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(untagged)]
enum ActionConfigLayout<T> {
	KeyOnly(KeyChord),
	TaggedFields(T),
}

impl<T> From<ActionConfigLayout<T>> for ActionConfig
where
	ActionConfig: From<T>,
	T: ConstActionType,
{
	fn from(layout: ActionConfigLayout<T>) -> Self {
		match layout {
			ActionConfigLayout::KeyOnly(key_chord) => {
				ActionConfig::new_default_inner(T::ACTION_TYPE, key_chord)
			}
			ActionConfigLayout::TaggedFields(action_config_inner) => action_config_inner.into(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
enum ActionConfigOrList<T> {
	Single(ActionConfigLayout<T>),
	List(Vec<ActionConfigLayout<T>>),
}

impl<T> From<ActionConfigOrList<T>> for Vec<ActionConfig>
where
	ActionConfig: From<T>,
	T: ConstActionType,
{
	fn from(config_or_list: ActionConfigOrList<T>) -> Self {
		match config_or_list {
			ActionConfigOrList::Single(action_config_layout) => vec![action_config_layout.into()],
			ActionConfigOrList::List(action_config_layouts) => action_config_layouts
				.into_iter()
				.map(|action_config_layout| action_config_layout.into())
				.collect(),
		}
	}
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KeyMapContextConfig {
	map: [Vec<ActionConfig>; ActionType::COUNT],
}

impl KeyMapContextConfig {
	pub fn new(key_map_context: &HashMap<KeyChord, Action>) -> Self {
		let mut key_map_context_config = Self::default();
		for (key_chord, &action) in key_map_context.iter() {
			key_map_context_config.map[action.discriminant() as usize]
				.push(ActionConfig::new(action, *key_chord));
		}
		key_map_context_config
	}

	pub fn values(&self) -> impl Iterator<Item = &ActionConfig> {
		self.map.iter().flat_map(|val| val.iter())
	}
}

impl Serialize for KeyMapContextConfig {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut map = serializer.serialize_map(Some(self.map.len()))?;
		for (action_type, action_configs) in ActionType::iter().zip(self.map.iter()) {
			match action_configs.len() {
				0 => continue,
				1 => {
					map.serialize_entry(&action_type, &action_configs[0])?;
				}
				_ => {
					map.serialize_entry(&action_type, action_configs)?;
				}
			}
		}
		map.end()
	}
}

impl<'de> Deserialize<'de> for KeyMapContextConfig {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_map(KeyMapContextConfigVisitor)
	}
}
