use super::View;
use color_eyre::eyre;
use std::iter::{self, repeat_n};

use crate::{
	config::{Action, Context, KeyChord},
	state::{AppState, FileBrowserItem},
	tui::input_handler::InputHandler,
};

use ratatui::{
	layout::{Constraint, Layout, Position},
	prelude::{Buffer, Rect},
	text::Span,
	widgets::{StatefulWidget, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTreeView;

impl FileTreeView {
	fn go_back(&self, state: &mut AppState) -> eyre::Result<()> {
		if let Some(parent) = state.file_browser.parent() {
			let prev_directory = state.file_browser.directory.clone();
			state.file_browser.open_directory(&parent)?;

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

		Ok(())
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
				state.file_browser.open_directory(&directory)?;
			}
		}

		Ok(())
	}

	fn go_to(&self, state: &mut AppState, target: u16) {
		let available_lines = state.file_browser.items.len();

		state
			.file_browser
			.cursor
			.set_y((target as u16).max(0) as u16)
			.update_pos(iter::repeat_n(1, available_lines))
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
		let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::FileBrowser)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		else {
			return Ok(false);
		};

		let line = state.file_browser.cursor.pos().y;

		match action {
			Action::Cancel => self.go_back(state)?,
			Action::Confirm => self.open_item(state, line.into())?,
			Action::MoveCursorX { amount } if amount > 0 => self.open_item(state, line.into())?,
			Action::MoveCursorX { amount } if amount < 0 => self.go_back(state)?,
			Action::SetCursorY { y } => self.go_to(state, y),
			Action::MoveCursorY { amount } => {
				if !(amount.is_negative() && amount.abs() > line as i16) {
					self.go_to(state, (line as i16 + amount) as u16)
				}
			}

			_ => return Ok(false),
		}

		Ok(true)
	}
}

impl StatefulWidget for FileTreeView {
	type State = AppState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		state.cursor.set_render_origin(None);

		let [top_line, content] =
			Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).areas(area);
		state.screen_size = Position::new(content.width, content.height);
		let line_count = content.height.min(state.file_browser.items.len() as u16);
		let line = state.file_browser.cursor.pos().y as usize;
		let constraints = Constraint::from_lengths(repeat_n(1, line_count as usize));
		let layout = Layout::vertical(constraints).split(content);

		Span::styled(
			state.file_browser.directory.to_str().unwrap_or_default(),
			state.config.theme.file_browser_parent_directory,
		)
		.render(top_line, buf);

		for (index, item) in state
			.file_browser
			.items
			.iter()
			.enumerate()
			.skip(state.file_browser.cursor.scroll().y as usize)
			.take(line_count as usize)
		{
			let style = if line == index {
				match item {
					FileBrowserItem::Song(_) => state.config.theme.file_browser_highlight_file,
					FileBrowserItem::Directory(_) => {
						state.config.theme.file_browser_highlight_directory
					}
				}
			} else {
				match item {
					FileBrowserItem::Song(_) => state.config.theme.file_browser_file,
					FileBrowserItem::Directory(_) => state.config.theme.file_browser_directory,
				}
			};

			let icon = match item {
				FileBrowserItem::Song(_) => " ",
				FileBrowserItem::Directory(_) => " ",
			};

			Span::styled(format!("{} {}", icon, item.name()), style).render(
				layout[index - state.file_browser.cursor.scroll().y as usize],
				buf,
			);
		}
	}
}
