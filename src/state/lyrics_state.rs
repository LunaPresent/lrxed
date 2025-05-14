use std::fs::File;

use color_eyre::eyre;
use ratatui::layout::Position;

use crate::lyrics::{Lyrics, TimeIndex, TimeIndexHint};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub time_index: TimeIndex,
	pub time_index_hint: TimeIndexHint,
	pub screen_size: Position,
}

impl LyricsState {
	pub fn load_file(&mut self, file: File) -> eyre::Result<()> {
		self.lyrics = Lyrics::from_file(file)?;
		self.time_index = TimeIndex::new(self.lyrics.lines().iter());
		self.time_index_hint = TimeIndexHint::default();
		Ok(())
	}
}
