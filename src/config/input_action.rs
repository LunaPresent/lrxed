use color_eyre::eyre;
use oprabeli::config::TryConvertAction;
use serde::{Deserialize, Serialize};

use crate::command::Command;
use crate::event::AppEvent;
use crate::util::QuadDirection;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(untagged)]
pub enum InputAction {
	Static(StaticInputAction),
	Command(Command),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StaticInputAction {
	KeyHelp,
	Quit,
	Confirm,
	Cancel,
	Yes,
	No,
	CursorUp,
	CursorDown,
	CursorLeft,
	CursorRight,
	CursorTop,
	CursorBottom,
	CursorStart,
	CursorEnd,
	ScrollUp,
	ScrollDown,
	ScrollHalfPageUp,
	ScrollHalfPageDown,
	ScrollFullPageUp,
	ScrollFullPageDown,
	Save,
	Undo,
	Redo,
	CursorToPlaying,
	CursorToPlayingLine,
	SeekToCursor,
	SeekToCursorLine,
	Sync,
	OpenInEditor,
	OpenFileOrDirectory,
	LeaveDirectory,
}

impl TryConvertAction for InputAction {
	type Event = AppEvent;
	type Error = eyre::Report;

	fn try_to_event(&self) -> Result<Self::Event, Self::Error> {
		Ok(match *self {
			InputAction::Static(action) => action.into_app_event(),
			InputAction::Command(cmd) => cmd.try_into_app_event()?,
		})
	}
}

impl StaticInputAction {
	pub fn into_app_event(self) -> AppEvent {
		match self {
			Self::KeyHelp => AppEvent::KeyHelp,
			Self::Quit => AppEvent::Quit,
			Self::Confirm => AppEvent::Confirm,
			Self::Cancel => AppEvent::Cancel,
			Self::Yes => AppEvent::Yes,
			Self::No => AppEvent::No,
			Self::CursorUp => AppEvent::MoveCursorBy {
				direction: QuadDirection::Up,
				amount: 1,
			},
			Self::CursorDown => AppEvent::MoveCursorBy {
				direction: QuadDirection::Down,
				amount: 1,
			},
			Self::CursorLeft => AppEvent::MoveCursorBy {
				direction: QuadDirection::Left,
				amount: 1,
			},
			Self::CursorRight => AppEvent::MoveCursorBy {
				direction: QuadDirection::Right,
				amount: 1,
			},
			Self::CursorTop => AppEvent::MoveCursorTo {
				x: None,
				y: Some(0),
			},
			Self::CursorBottom => AppEvent::MoveCursorTo {
				x: None,
				y: Some(u16::MAX),
			},
			Self::CursorStart => AppEvent::MoveCursorTo {
				x: Some(0),
				y: None,
			},
			Self::CursorEnd => AppEvent::MoveCursorTo {
				x: Some(u16::MAX),
				y: None,
			},
			Self::ScrollUp => AppEvent::ScrollBy {
				direction: QuadDirection::Up,
				amount: 1,
			},
			Self::ScrollDown => AppEvent::ScrollBy {
				direction: QuadDirection::Down,
				amount: 1,
			},
			Self::ScrollHalfPageUp => AppEvent::MoveCursorByRelative {
				direction: QuadDirection::Up,
				fraction: 0.5,
			},
			Self::ScrollHalfPageDown => AppEvent::MoveCursorByRelative {
				direction: QuadDirection::Down,
				fraction: 0.5,
			},
			Self::ScrollFullPageUp => AppEvent::MoveCursorByRelative {
				direction: QuadDirection::Up,
				fraction: 1.,
			},
			Self::ScrollFullPageDown => AppEvent::MoveCursorByRelative {
				direction: QuadDirection::Down,
				fraction: 1.,
			},
			Self::Save => AppEvent::Save,
			Self::Undo => AppEvent::Undo,
			Self::Redo => AppEvent::Redo,
			Self::CursorToPlaying => AppEvent::CursorToPlaying,
			Self::CursorToPlayingLine => AppEvent::CursorToPlayingLine,
			Self::SeekToCursor => AppEvent::SeekToCursor,
			Self::SeekToCursorLine => AppEvent::SeekToCursorLine,
			Self::Sync => AppEvent::Sync,
			Self::OpenInEditor => AppEvent::OpenInEditor,
			Self::OpenFileOrDirectory => AppEvent::OpenFileOrDirectory,
			Self::LeaveDirectory => AppEvent::LeaveDirectory,
		}
	}
}
