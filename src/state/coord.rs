use std::cmp;

use ratatui::layout::Position;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
	pub x: u16,
	pub y: u16,
}

impl Into<Position> for Coord {
	fn into(self) -> Position {
		Position {
			x: self.x,
			y: self.y,
		}
	}
}
