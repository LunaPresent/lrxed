use std::{
	fs::{File, OpenOptions},
	time::Duration,
};

use color_eyre::eyre;
use ratatui::layout::Position;

use crate::lyrics::{
	Lyrics, TimeIndex, TimeIndexHint,
	editing::{Edit, EditAction, History},
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsState {
	pub lyrics: Lyrics,
	pub lrc_file_path: String,
	pub time_index: TimeIndex,
	pub time_index_hint: TimeIndexHint,
	pub screen_size: Position,
	pub history: History,
	pub changed: bool,
}

impl LyricsState {
	pub fn load_file(&mut self, lrc_path: &str) -> eyre::Result<()> {
		self.lrc_file_path = lrc_path.to_owned();
		self.lyrics = Lyrics::from_file(File::open(lrc_path)?)?;
		self.time_index = TimeIndex::new(self.lyrics.lines().iter());
		self.time_index_hint = TimeIndexHint::default();
		Ok(())
	}

	pub fn write_to_file(&mut self) -> eyre::Result<()> {
		self.lyrics.write_to_file(
			OpenOptions::new()
				.read(false)
				.write(true)
				.create(true)
				.truncate(true)
				.open(&self.lrc_file_path)?,
		)?;
		self.changed = false;
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
		edit.execute_forwards(&mut self.lyrics, &mut self.time_index)?;
		self.history.push(edit);
		self.changed = true;

		Ok(())
	}
}
