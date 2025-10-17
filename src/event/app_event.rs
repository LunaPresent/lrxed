use core::time::Duration;

use oprabeli::ratatui::layout::Rect;
use serde_with::chrono;

use crate::util::{BoolModifier, QuadDirection};

#[derive(Debug, Clone, PartialEq)]
pub enum AppEvent {
	KeyHelp,
	Quit,
	Suspend,
	Confirm,
	Cancel,
	Yes,
	No,
	Save,
	Undo,
	Redo,
	MoveCursorBy {
		direction: QuadDirection,
		amount: u16,
	},
	MoveCursorByRelative {
		direction: QuadDirection,
		fraction: f32,
	},
	MoveCursorTo {
		x: Option<u16>,
		y: Option<u16>,
	},
	CursorToPlaying,
	CursorToPlayingLine,
	ScrollBy {
		direction: QuadDirection,
		amount: u16,
	},
	ScrollByRelative {
		direction: QuadDirection,
		fraction: f32,
	},
	ScrollTo(Rect),
	SeekBy(chrono::TimeDelta),
	SeekByRelative(f32),
	SeekTo(Duration),
	SeekToRelative(f32),
	SeekToCursor,
	SeekToCursorLine,
	SetPlayback(BoolModifier),
	ChangeVolumeBy(i16),
	ChangeVolumeTo(u16),
	ChangeSpeedBy(i16),
	ChangeSpeedTo(u16),
	ChangeTimestampBy(chrono::TimeDelta),
	ChangeTimestampTo(Duration),
	Sync,
	OpenInEditor,
	OpenFileOrDirectory,
	LeaveDirectory,
	UpdateKeymap,
}
