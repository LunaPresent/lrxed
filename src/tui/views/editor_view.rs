use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
	layout::{Constraint, Layout},
	widgets::StatefulWidget,
};

use crate::{
	state::AppState,
	tui::{
		input_handler::InputHandler,
		widgets::{LyricsWidget, PlaybackWidget},
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorView;

impl InputHandler for EditorView {
	type State = AppState;

	fn handle_input(&mut self, key: &KeyEvent, state: &mut AppState) -> eyre::Result<bool> {
		match key.code {
			KeyCode::Char(c @ '0'..='9') => {
				state
					.audio_state
					.seek_relative_or_ignore((c as u32 - '0' as u32) as f32 / 10.)?;
				Ok(true)
			}
			_ => Ok(false),
		}
	}
}

impl StatefulWidget for EditorView {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		let layout = Layout::vertical([Constraint::Min(4), Constraint::Length(4)]);
		let [lyrics_area, playback_area] = layout.areas(area);

		LyricsWidget.render(lyrics_area, buf, state);
		PlaybackWidget.render(playback_area, buf, state);
	}
}
