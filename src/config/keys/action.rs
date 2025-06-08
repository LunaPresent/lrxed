#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use strum::{EnumCount, EnumDiscriminants, EnumIter};

use super::KeyChord;

macro_rules! define_actions {
	($($name:ident $({ $value:ident: $data:ty })?),+ $(,)?) => {

#[derive(Debug, Clone, Copy, PartialEq, EnumDiscriminants)]
#[strum_discriminants(name(ActionType))]
#[strum_discriminants(derive(EnumCount, EnumIter, Serialize, Deserialize))]
#[strum_discriminants(serde(rename_all = "kebab-case"))]
pub enum Action {
	$($name$({ $value: $data })?),+
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionConfig {
	$($name { key: KeyChord, $($value: $data,)? }),+
}

impl ActionConfig {
	pub fn new(action: Action, key: KeyChord) -> Self {
		match action {
			$(Action::$name$({ $value })? => Self::$name{ $($value,)? key }),+
		}
	}

	pub fn action(&self) -> Action {
		match self {
			$(Self::$name { $($value,)? key: _ } => Action::$name$({ $value: *$value})?),+
		}
	}

	pub fn key(&self) -> KeyChord {
		match self {
			$(Self::$name { $($value: _,)? key } => *key),+
		}
	}
}

	};
}

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
