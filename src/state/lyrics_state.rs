use std::{fs::File, time::Duration};

use color_eyre::eyre;
use ratatui::layout::Position;

use crate::lyrics::{
	Lyrics, TimeIndex, TimeIndexHint,
	editing::{Edit, EditAction, History},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub time_index: TimeIndex,
	pub time_index_hint: TimeIndexHint,
	pub screen_size: Position,
	pub history: History,
}

impl LyricsState {
	pub fn load_file(&mut self, file: File) -> eyre::Result<()> {
		self.lyrics = Lyrics::from_file(file)?;
		self.time_index = TimeIndex::new(self.lyrics.lines().iter());
		self.time_index_hint = TimeIndexHint::default();
		Ok(())
	}

	pub fn set_timestamp(
		&mut self,
		cursor_pos: Position,
		timestamp: Option<Duration>,
	) -> eyre::Result<()> {
		let prev_val = self
			.lyrics
			.time_at_cursor(cursor_pos.x, cursor_pos.y)
			.map(|x| x.time());
		let edit = Edit::new(
			EditAction::SetTimestamp {
				idx: cursor_pos.y,
				timestamp,
			},
			EditAction::SetTimestamp {
				idx: cursor_pos.y,
				timestamp: prev_val,
			},
		);
		edit.execute_forwards(&mut self.lyrics)?;
		self.history.push(edit);
		Ok(())
	}
}
