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

impl InputHandler for FileTreeView {
	type State = AppState;

	fn handle_input(self, key_chord: KeyChord, state: &mut AppState) -> eyre::Result<bool> {
		let line = state.file_browser.cursor.pos().y as usize;

		if let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::Editor)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		{
			match action {
				Action::MoveCursorY(amount) => {
					let available_lines = state
						.screen_size
						.y
						.min(state.file_browser.items.len() as u16);

					state
						.file_browser
						.cursor
						.set_y((state.file_browser.cursor.pos().y as i16 + amount).max(0) as u16)
						.update_pos((0..available_lines).map(|_| 1))
						.update_scroll(
							Position::new(0, state.file_browser.items.len() as u16),
							state.screen_size,
							state.config.settings.scrolloff,
						);
				}
				Action::OpenInEditor => match state.file_browser.items[line].clone() {
					FileBrowserItem::Song(song) => {
						state.audio.audio_player =
							Some(state.audio.audio_device.try_play(song.mp3_file.clone())?);

						if let Some(lyrics) = &song.lrc_file {
							state.lyrics.load_file(lyrics.clone()).unwrap();
						}

						state.active_view = View::Editor;
					}
					FileBrowserItem::Directory(directory) => {
						state.file_browser.open_directory(&directory);
					}
				},
				Action::Back => {
					if let Some(parent) = state.file_browser.parent() {
						state.file_browser.open_directory(&parent);
					}
				}
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

			match item {
				FileBrowserItem::Song(file) => {
					let meta = file.meta.as_ref().unwrap();
					let text = format!("  {} - {}", meta.artist, meta.title);
					Span::styled(text, style).render(layout[index], buf);
				}
				FileBrowserItem::Directory(directory) => {
					let text = format!("  {}", directory.to_str().unwrap());
					Span::styled(text, style).render(layout[index], buf);
				}
			}
		}
	}
}
