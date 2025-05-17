use std::{
	cmp::{max, min},
	time::Duration,
};

use color_eyre::eyre::{self, OptionExt};
use ratatui::{
	layout::{Constraint, Layout, Position},
	widgets::StatefulWidget,
};

use crate::{
	config::Action,
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

	fn handle_input(&mut self, action: Action, state: &mut AppState) -> eyre::Result<bool> {
		match action {
			Action::MoveCursorY(offset) => {
				state
					.cursor
					.set_y(max(state.cursor.pos().y as i16 + offset, 0) as u16)
					.update_pos(state.lyrics.lyrics.line_lenghts())
					.update_scroll(
						Position::new(
							state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
							state.lyrics.lyrics.line_count(),
						),
						state.lyrics.screen_size,
						state.config.settings.scrolloff,
					);
				state.cursor.set_y(state.cursor.pos().y);
				Ok(true)
			}
			Action::MoveCursorX(offset) => {
				state
					.cursor
					.set_x(max(state.cursor.pos().x as i16 + offset, 0) as u16)
					.update_pos(state.lyrics.lyrics.line_lenghts())
					.update_scroll(
						Position::new(
							state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
							state.lyrics.lyrics.line_count(),
						),
						state.lyrics.screen_size,
						state.config.settings.scrolloff,
					);
				state.cursor.set_x(state.cursor.pos().x);
				Ok(true)
			}
			Action::SetCursorY(y) => {
				state
					.cursor
					.set_y(y)
					.update_pos(state.lyrics.lyrics.line_lenghts())
					.update_scroll(
						Position::new(
							state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
							state.lyrics.lyrics.line_count(),
						),
						state.lyrics.screen_size,
						state.config.settings.scrolloff,
					);
				Ok(true)
			}
			Action::SetCursorX(x) => {
				state
					.cursor
					.set_x(x)
					.update_pos(state.lyrics.lyrics.line_lenghts())
					.update_scroll(
						Position::new(
							state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
							state.lyrics.lyrics.line_count(),
						),
						state.lyrics.screen_size,
						state.config.settings.scrolloff,
					);
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
						.cursor
						.set_y(y)
						.update_pos(state.lyrics.lyrics.line_lenghts())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
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
						.cursor
						.set_y(y)
						.update_pos(state.lyrics.lyrics.line_lenghts())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
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

				if let Some(timestamp) = state
					.lyrics
					.lyrics
					.time_at_cursor(state.cursor.pos().x, state.cursor.pos().y)
				{
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

				if let Some(timestamp) = state.lyrics.lyrics.time_at_line(state.cursor.pos().y) {
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
			Action::Undo => {
				state.lyrics.history.undo(&mut state.lyrics.lyrics)?;
				Ok(true)
			}
			Action::Redo => {
				state.lyrics.history.redo(&mut state.lyrics.lyrics)?;
				Ok(true)
			}
			Action::SyncTimestamp => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				state
					.lyrics
					.set_timestamp(state.cursor.pos(), Some(player.position()))?;
				state
					.cursor
					.set_y(state.cursor.pos().y + 1)
					.update_pos(state.lyrics.lyrics.line_lenghts())
					.update_scroll(
						Position::new(
							state.lyrics.lyrics.line_lenghts().max().unwrap_or_default(),
							state.lyrics.lyrics.line_count(),
						),
						state.lyrics.screen_size,
						state.config.settings.scrolloff,
					);
				state.cursor.set_y(state.cursor.pos().y);
				Ok(true)
			}
			Action::AdjustTimestamp(centis) => {
				let current_timestamp = state
					.lyrics
					.lyrics
					.time_at_cursor(state.cursor.pos().x, state.cursor.pos().y)
					.ok_or_eyre("No timestamp at cursor")?
					.time();
				let timestamp = Duration::from_millis(max(
					current_timestamp.as_millis() as i32 + centis * 10,
					0,
				) as u64);
				state
					.lyrics
					.set_timestamp(state.cursor.pos(), Some(timestamp))?;
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
