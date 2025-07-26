use color_eyre::eyre;
use ratatui::layout::Position;

use std::{
	fs::{self, File, OpenOptions},
	io::{BufReader, BufWriter},
	path::PathBuf,
	time::Duration,
};

use crate::{
	lyrics::{
		Lyrics, TimeIndex, TimeIndexHint,
		editing::{Edit, EditAction, History},
	},
	song::Song,
};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct SongState {
	pub song: Song,
	pub time_index: TimeIndex,
	pub time_index_hint: TimeIndexHint,
	pub history: History,
	pub changed: bool,
}

impl SongState {
	pub fn load_from_song(&mut self, song: Song) -> eyre::Result<bool> {
		self.song = song;
		self.time_index = TimeIndex::new(self.song.lyrics.lines().iter());
		self.time_index_hint = TimeIndexHint::default();

		Ok(true)
	}

	pub fn load_file_if_exists(&mut self, lrc_path: PathBuf) -> eyre::Result<bool> {
		let exists = if lrc_path.exists() {
			self.song
				.lyrics
				.read_overwrite(BufReader::new(File::open(&lrc_path)?))?;

			true
		} else {
			self.song.lyrics = Lyrics::default();
			false
		};
		self.time_index = TimeIndex::new(self.song.lyrics.lines().iter());
		self.time_index_hint = TimeIndexHint::default();
		self.song.lrc_file = lrc_path;
		Ok(exists)
	}

	pub fn write_to_file(&mut self, replace_txt_file: bool) -> eyre::Result<()> {
		if self
			.song
			.lrc_file
			.extension()
			.is_some_and(|ext| ext == "txt")
		{
			let new_file_path = self.song.lrc_file.with_extension("lrc");

			if replace_txt_file {
				fs::rename(&self.song.lrc_file, &new_file_path)?;
			}

			self.song.lrc_file = new_file_path;
		}

		self.song.lyrics.write_to(&mut BufWriter::new(
			OpenOptions::new()
				.read(false)
				.write(true)
				.create(true)
				.truncate(true)
				.open(&self.song.lrc_file)?,
		))?;

		self.song.has_file = true;
		self.changed = false;

		Ok(())
	}

	pub fn undo(&mut self) -> eyre::Result<()> {
		self.changed = true;

		self.history
			.undo(&mut self.song.lyrics, &mut self.time_index)
	}

	pub fn redo(&mut self) -> eyre::Result<()> {
		self.changed = true;

		self.history
			.redo(&mut self.song.lyrics, &mut self.time_index)
	}

	pub fn set_timestamp(
		&mut self,
		cursor_pos: Position,
		timestamp: Option<Duration>,
	) -> eyre::Result<()> {
		let prev_val = self
			.song
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
		edit.execute_forwards(&mut self.song.lyrics, &mut self.time_index)?;
		self.history.push(edit);
		self.changed = true;

		Ok(())
	}
}
