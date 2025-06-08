use std::{collections::HashMap, fmt};

use serde::{Deserialize, Serialize, de::Visitor, ser::SerializeMap};
use strum::{EnumCount, IntoDiscriminant, IntoEnumIterator};

use super::{
	Action, KeyChord,
	action::{ActionConfig, ActionType},
};

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

struct KeyMapContextConfigVisitor;

impl<'de> Visitor<'de> for KeyMapContextConfigVisitor {
	type Value = KeyMapContextConfig;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a map with ActionType as key and ActionConfig as value")
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: serde::de::MapAccess<'de>,
	{
		let mut key_map = Self::Value::default();
		while let Some((key, value)) = access.next_entry::<ActionType, _>()? {
			key_map.map[key as usize] = value;
		}
		Ok(key_map)
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
