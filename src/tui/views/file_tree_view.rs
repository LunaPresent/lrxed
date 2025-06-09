use super::View;
use color_eyre::eyre;
use std::iter::repeat_n;

use crate::{
	config::{Action, Context, KeyChord},
	state::{AppState, FileBrowserItem},
	tui::input_handler::InputHandler,
};

use ratatui::{
	layout::{Constraint, Layout, Position},
	prelude::{Buffer, Rect},
	style::{Color, Style, Stylize},
	text::Span,
	widgets::{StatefulWidget, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTreeView;

impl FileTreeView {
	fn go_back(&self, state: &mut AppState) {
		if let Some(parent) = state.file_browser.parent() {
			let prev_directory = state.file_browser.directory.clone();
			state.file_browser.open_directory(&parent);

			let line = state
				.file_browser
				.items
				.iter()
				.enumerate()
				.find(|(_, item)| **item == FileBrowserItem::Directory(prev_directory.clone()))
				.map(|(index, _)| index)
				.unwrap_or(0);

			self.go_to(state, line as u16);
		}
	}

	fn open_item(&self, state: &mut AppState, line: usize) -> eyre::Result<()> {
		match state.file_browser.items[line].clone() {
			FileBrowserItem::Song(song) => {
				state.audio.audio_player =
					Some(state.audio.audio_device.try_play(song.mp3_file.clone())?);

				if let Some(lyrics) = song.lrc_file.as_ref() {
					state.lyrics.load_file(lyrics.clone()).unwrap();
				}

				state.active_view = View::Editor;
			}
			FileBrowserItem::Directory(directory) => {
				self.go_to(state, 0);
				state.file_browser.open_directory(&directory);
			}
		}

		Ok(())
	}

	fn go_to(&self, state: &mut AppState, target: u16) {
		let available_lines = state
			.screen_size
			.y
			.min(state.file_browser.items.len() as u16);

		state
			.file_browser
			.cursor
			.set_y((target as u16).max(0) as u16)
			.update_pos((0..available_lines).map(|_| 1))
			.update_scroll(
				Position::new(0, state.file_browser.items.len() as u16),
				state.screen_size,
				state.config.settings.scrolloff,
			);
	}
}

impl InputHandler for FileTreeView {
	type State = AppState;

	fn handle_input(self, key_chord: KeyChord, state: &mut AppState) -> eyre::Result<bool> {
		let line = state.file_browser.cursor.pos().y;

		if let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::FileBrowser)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		{
			match action {
				Action::SetCursorY(position) => self.go_to(state, position),
				Action::MoveCursorY(amount) => self.go_to(state, (line as i16 + amount) as u16),

				Action::Cancel => self.go_back(state),
				Action::OpenInEditor => self.open_item(state, line.into())?,
				Action::MoveCursorX(amount) if amount > 0 => self.open_item(state, line.into())?,
				Action::MoveCursorX(amount) if amount < 0 => self.go_back(state),

				_ => return Ok(false),
			}

			Ok(true)
		} else {
			Ok(false)
		}
	}
}

impl StatefulWidget for FileTreeView {
	type State = AppState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		state.screen_size = Position::new(area.width, area.height);

		let line_count = area.height.min(state.file_browser.items.len() as u16);
		let line = state.file_browser.cursor.pos().y as usize;
		let constraints = Constraint::from_lengths(repeat_n(1, line_count as usize));
		let layout = Layout::vertical(constraints).split(area);

		for (index, item) in state
			.file_browser
			.items
			.iter()
			.skip(state.file_browser.cursor.scroll().y as usize)
			.take(line_count as usize)
			.enumerate()
		{
			let mut style = Style::default();

			if line == index {
				style = style.bold().black().bg(Color::Blue);
			}

			let icon = match item {
				FileBrowserItem::Song(_) => " ",
				FileBrowserItem::Directory(_) => " ",
			};

			Span::styled(format!("{} {}", icon, item.name()), style).render(layout[index], buf);
		}
	}
}
