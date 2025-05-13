use std::{
	cmp::{max, min},
	fs::File,
};

use color_eyre::eyre;

use crate::lyrics::{Lyrics, TimeIndex, TimeIndexHint};

use super::Coord;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub time_index: TimeIndex,
	pub time_index_hint: TimeIndexHint,
	pub cursor: Coord,
	pub cursor_target: Coord,
	pub scroll: Coord,
	pub bufsize: Coord,
}

impl LyricsState {
	pub fn load_file(&mut self, file: File) -> eyre::Result<()> {
		self.lyrics = Lyrics::from_file(file)?;
		self.time_index = TimeIndex::new(self.lyrics.lines().iter());
		self.time_index_hint = TimeIndexHint::default();
		Ok(())
	}

	pub fn set_cursor_x(&mut self, x: u16) {
		self.cursor_target.x = x;
		self.cursor.y = min(self.lyrics.line_count() - 1, self.cursor_target.y);
		self.cursor.x = min(
			self.cursor_target.x,
			max(self.lyrics.lines()[self.cursor.y as usize].text().len(), 1) as u16 - 1,
		);
	}

	pub fn set_cursor_y(&mut self, y: u16, scrolloff: u16) {
		self.cursor_target.y = y;
		self.cursor.y = min(self.lyrics.line_count() - 1, self.cursor_target.y);
		self.cursor.x = min(
			self.cursor_target.x,
			max(self.lyrics.lines()[self.cursor.y as usize].text().len(), 1) as u16 - 1,
		);
		self.autoscroll(scrolloff);
	}

	pub fn set_cursor_pos(&mut self, pos: Coord, scrolloff: u16) {
		self.cursor_target = pos;
		self.cursor.y = min(self.lyrics.line_count() - 1, pos.y);
		self.cursor.x = min(
			pos.x,
			max(self.lyrics.lines()[self.cursor.y as usize].text().len(), 1) as u16 - 1,
		);
		self.autoscroll(scrolloff);
	}

	pub fn autoscroll(&mut self, scrolloff: u16) {
		self.autoscroll_up(min(scrolloff, self.lyrics.line_count() / 2));
		self.autoscroll_down(min(scrolloff, self.lyrics.line_count() / 2));
	}

	fn autoscroll_up(&mut self, scrolloff: u16) {
		self.scroll.y = min(self.cursor.y - min(self.cursor.y, scrolloff), self.scroll.y);
	}

	fn autoscroll_down(&mut self, scrolloff: u16) {
		let mut scroll_bottom = self.scroll.y + self.bufsize.y;
		scroll_bottom = max(
			min(self.cursor.y + scrolloff + 1, self.lyrics.line_count()),
			scroll_bottom,
		);
		self.scroll.y = scroll_bottom - self.bufsize.y;
	}
}
