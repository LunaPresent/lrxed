use std::{
	cmp::{max, min},
	time::Duration,
};

use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Layout},
	widgets::StatefulWidget,
};

use crate::{
	config::Action,
	state::{AppState, Coord},
	tui::{
		input_handler::InputHandler,
		widgets::{LyricsWidget, PlaybackWidget},
	},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorView;

impl InputHandler for EditorView {
	type State = AppState;

	fn handle_input(&mut self, action: Action, state: &mut AppState) -> eyre::Result<bool> {
		match action {
			Action::MoveCursorY(offset) => {
				state.lyrics.set_cursor_y(
					max(state.lyrics.cursor.y as i16 + offset, 0) as u16,
					state.config.settings.scrolloff,
				);
				Ok(true)
			}
			Action::MoveCursorX(offset) => {
				state
					.lyrics
					.set_cursor_x(max(state.lyrics.cursor.x as i16 + offset, 0) as u16);
				state.lyrics.cursor_target.x = state.lyrics.cursor.x;
				Ok(true)
			}
			Action::SetCursorY(y) => {
				state
					.lyrics
					.set_cursor_y(y, state.config.settings.scrolloff);
				Ok(true)
			}
			Action::SetCursorX(x) => {
				state.lyrics.set_cursor_x(x);
				Ok(true)
			}
			Action::CursorToPlaying => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;

				// TODO: get cursor x position from time and set it
				let time;
				(time, state.lyrics.time_index_hint) = state
					.lyrics
					.time_index
					.find_seq(player.position(), state.lyrics.time_index_hint);

				if let Some(y) = time.line_num {
					state
						.lyrics
						.set_cursor_pos(Coord { x: 0, y }, state.config.settings.scrolloff);
				}
				Ok(true)
			}
			Action::CursorToPlayingLine => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;

				let time;
				(time, state.lyrics.time_index_hint) = state
					.lyrics
					.time_index
					.find_seq(player.position(), state.lyrics.time_index_hint);

				if let Some(y) = time.line_num {
					state
						.lyrics
						.set_cursor_pos(Coord { x: 0, y }, state.config.settings.scrolloff);
				}
				Ok(true)
			}
			Action::SeekRelative(relative_pos) => {
				let pos = state.audio.seek_relative(relative_pos)?;
				if let Some(time) = pos {
					(_, state.lyrics.time_index_hint) = state.lyrics.time_index.find_random(time);
				}
				Ok(true)
			}
			Action::SeekBackwards => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let pos = player.position();
				player.seek(
					pos - min(
						Duration::from_secs_f32(state.config.settings.jump_seconds),
						pos,
					),
				)?;
				Ok(true)
			}
			Action::SeekForwards => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				player.seek(
					player.position() + Duration::from_secs_f32(state.config.settings.jump_seconds),
				)?;
				Ok(true)
			}
			Action::SeekToCursor => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;

				if let Some(timestamp) = state.lyrics.lyrics.time_at_cursor(state.lyrics.cursor) {
					player.seek(timestamp.time() + Duration::from_millis(1))?;
					(_, state.lyrics.time_index_hint) =
						state.lyrics.time_index.find_random(timestamp.time());
				}
				Ok(true)
			}
			Action::SeekToCursorLine => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;

				if let Some(timestamp) = state.lyrics.lyrics.time_at_line(state.lyrics.cursor.y) {
					player.seek(timestamp.time() + Duration::from_millis(1))?;
					(_, state.lyrics.time_index_hint) =
						state.lyrics.time_index.find_random(timestamp.time());
				}
				Ok(true)
			}
			Action::TogglePause => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				player.set_paused(!player.is_paused());
				Ok(true)
			}
			Action::ChangeVolume(pct) => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let volume = (player.volume() * 100. + 0.5) as i16 + pct;
				player.set_volume(min(max(volume, 0), 100) as f32 / 100.);
				Ok(true)
			}
			Action::ChangeSpeed(pct) => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let speed = (player.speed() * 100. + 0.5) as i16 + pct;
				player.set_speed(min(max(speed, 50), 200) as f32 / 100.);
				Ok(true)
			}
			Action::ResetSpeed => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				player.set_speed(1.);
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
