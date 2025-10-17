use core::{fmt, str};

use serde_with::SerializeDisplay;

use super::ParseCommandError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SerializeDisplay)]
pub enum CommandKind {
	Save,
	Playback,
	Seek,
	Volume,
	Speed,
	Timestamp,
}

impl fmt::Display for CommandKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Save => write!(f, "save"),
			Self::Playback => write!(f, "playback"),
			Self::Seek => write!(f, "seek"),
			Self::Volume => write!(f, "volume"),
			Self::Speed => write!(f, "speed"),
			Self::Timestamp => write!(f, "timestamp"),
		}
	}
}

impl str::FromStr for CommandKind {
	type Err = ParseCommandError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"save" | "write" | "w" => Ok(Self::Save),
			"playback" | "play" | "pl" => Ok(Self::Playback),
			"seek" | "se" => Ok(Self::Seek),
			"volume" | "vol" | "v" => Ok(Self::Volume),
			"speed" | "spd" | "sp" => Ok(Self::Speed),
			"timestamp" | "time" | "t" => Ok(Self::Timestamp),
			_ => Err(ParseCommandError::InvalidCommand(s.to_owned())),
		}
	}
}
