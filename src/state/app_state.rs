use std::{ffi::OsString, io::stdout};

use color_eyre::eyre;
use edit::Builder;
use ratatui::crossterm::{
	ExecutableCommand,
	terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::{
	lyrics::{
		Lyrics,
		editing::{Edit, EditAction},
	},
	tui::{Cursor, Modal, View},
};

use super::{AudioState, Config, LyricsState, ModalState};

pub struct AppState {
	pub audio: AudioState,
	pub lyrics: LyricsState,
	pub modal: ModalState,
	pub cursor: Cursor,
	pub config: Config,
	pub active_view: View,
	pub active_modal: Option<Modal>,
	pub refresh_term: bool,
	pub should_quit: bool,
}

impl AppState {
	pub fn new(initial_view: View) -> Self {
		Self {
			audio: Default::default(),
			lyrics: Default::default(),
			modal: Default::default(),
			cursor: Default::default(),
			config: Default::default(),
			active_view: initial_view,
			active_modal: None,
			refresh_term: false,
			should_quit: false,
		}
	}

	pub fn open_in_editor(&mut self) -> eyre::Result<()> {
		let mut buf = Vec::new();
		self.lyrics.lyrics.write_to(&mut buf)?;
		stdout().execute(LeaveAlternateScreen)?;

		let bytes = edit::edit_bytes_with_builder(
			&buf,
			Builder::new()
				.prefix(
					&self
						.lyrics
						.lrc_file_path
						.file_stem()
						.unwrap_or(Into::<OsString>::into("lyrics").as_os_str()),
				)
				.suffix(".lrc"),
		)?;

		stdout().execute(EnterAlternateScreen)?;
		self.refresh_term = true;

		if buf == bytes {
			return Ok(());
		}

		let edit = Edit::new(
			EditAction::RestoreState(bytes),
			EditAction::RestoreState(buf),
		);
		let result = edit.execute_forwards(&mut self.lyrics.lyrics, &mut self.lyrics.time_index);
		self.lyrics.history.push(edit);
		self.lyrics.changed = true;

		result
	}
}
