mod direction;

pub use direction::*;

use oprabeli::ratatui::layout::{Offset, Position, Rect};

pub trait IntoOffset {
	fn into_offset(self) -> Offset;
}

impl IntoOffset for Position {
	fn into_offset(self) -> Offset {
		Offset {
			x: self.x as i32,
			y: self.y as i32,
		}
	}
}

pub trait ResetOrigin {
	fn reset_origin(self) -> Self;
}

impl ResetOrigin for Rect {
	fn reset_origin(self) -> Self {
		Rect {
			x: 0,
			y: 0,
			width: self.width,
			height: self.height,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolModifier {
	SetTrue,
	SetFalse,
	Toggle,
}
