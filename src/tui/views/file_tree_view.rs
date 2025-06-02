use std::iter::repeat_n;

use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::{Style, Stylize},
	text::Span,
	widgets::{StatefulWidget, Widget},
};

use crate::{
	config::{Action, Context, KeyChord},
	state::AppState,
	tui::input_handler::InputHandler,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTreeView;

impl InputHandler for FileTreeView {
	type State = AppState;

	fn handle_input(self, key_chord: KeyChord, state: &mut AppState) -> eyre::Result<bool> {
		if let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::Editor)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		{
			match action {
				Action::Save => state.lyrics.write_to_file()?,
				Action::MoveCursorY(amount) => state.file_browser.selected_line += amount,
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
		let line = state.file_browser.selected_line;
		let items = state.file_browser.get_directory_contents();
		let constraints = Constraint::from_lengths(repeat_n(1, items.len()));
		let layout = Layout::vertical(constraints).split(area);

		for (index, item) in items.iter().enumerate() {
			let mut style = Style::default();

			if line == index as i16 {
				style = style.bold();
			}

			Span::styled(item.to_str().unwrap_or_default(), style).render(layout[index], buf);
		}
	}
}
