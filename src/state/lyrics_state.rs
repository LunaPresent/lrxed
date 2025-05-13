use std::cmp::{max, min};

use crate::lyrics::Lyrics;

use super::Coord;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub cursor: Coord,
	pub scroll_y: usize,
	pub bufsize: Coord,
}

impl LyricsState {
	pub fn cursor_to(&mut self, y: usize, scrolloff: usize) {
		self.cursor.y = min(self.lyrics.lines().len() - 1, y);
		self.autoscroll(scrolloff);
	}

	pub fn autoscroll(&mut self, scrolloff: usize) {
		self.autoscroll_up(min(scrolloff, self.lyrics.lines().len() / 2));
		self.autoscroll_down(min(scrolloff, self.lyrics.lines().len() / 2));
	}

	fn autoscroll_up(&mut self, scrolloff: usize) {
		self.scroll_y = min(self.cursor.y - min(self.cursor.y, scrolloff), self.scroll_y);
	}

	fn autoscroll_down(&mut self, scrolloff: usize) {
		let mut scroll_bottom = self.scroll_y + self.bufsize.y;
		scroll_bottom = max(
			min(self.cursor.y + scrolloff + 1, self.lyrics.lines().len()),
			scroll_bottom,
		);
		self.scroll_y = scroll_bottom - self.bufsize.y;
	}
}
