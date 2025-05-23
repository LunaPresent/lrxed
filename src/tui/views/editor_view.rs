use std::{
	cmp::{max, min},
	time::Duration,
	u16,
};

use color_eyre::eyre::{self, OptionExt};
use ratatui::{
	layout::{Constraint, Layout, Position},
	widgets::StatefulWidget,
};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::{
	audio::AudioPlayer,
	config::{Action, Context, KeyChord},
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

	fn handle_input(self, key_chord: KeyChord, state: &mut AppState) -> eyre::Result<bool> {
		if let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::Editor)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		{
			match action {
				Action::Save => {
					state.lyrics.write_to_file()?;
				}
				Action::MoveCursorY(offset) => {
					state
						.cursor
						.set_y(max(state.cursor.pos().y as i16 + offset, 0) as u16)
						.update_pos(state.lyrics.lyrics.line_widths())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
					state.cursor.set_y(state.cursor.pos().y);
				}
				Action::MoveCursorX(offset) => {
					let line = state.lyrics.lyrics.lines()[state.cursor.pos().y as usize].text();
					let x = if offset >= 0 {
						line.chars()
							.scan(0, |head, c| {
								let char_pos = *head as u16;
								let char_width = c.width().unwrap_or_default() as u16;
								*head += char_width;
								Some((char_pos, char_width))
							})
							.skip_while(|(char_pos, char_width)| {
								char_pos + char_width <= state.cursor.pos().x
							})
							.map(|(char_pos, _)| char_pos)
							.nth(offset as usize)
							.unwrap_or(u16::MAX)
					} else {
						line.chars()
							.rev()
							.scan(line.width() as u16, |head, c| {
								let char_width = c.width().unwrap_or_default() as u16;
								*head -= char_width;
								Some(*head)
							})
							.skip_while(|&char_pos| char_pos > state.cursor.pos().x)
							.nth(offset.abs() as usize)
							.unwrap_or_default()
					};

					state
						.cursor
						.set_x(max(x, 0) as u16)
						.update_pos(state.lyrics.lyrics.line_widths())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
					state.cursor.set_x(state.cursor.pos().x);
				}
				Action::SetCursorY(y) => {
					state
						.cursor
						.set_y(y)
						.update_pos(state.lyrics.lyrics.line_widths())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
				}
				Action::SetCursorX(x) => {
					state
						.cursor
						.set_x(x)
						.update_pos(state.lyrics.lyrics.line_widths())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
				}
				Action::CursorToPlaying => {
					let player = get_player(state)?;

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
							.update_pos(state.lyrics.lyrics.line_widths())
							.update_scroll(
								Position::new(
									state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
									state.lyrics.lyrics.line_count(),
								),
								state.lyrics.screen_size,
								state.config.settings.scrolloff,
							);
					}
				}
				Action::CursorToPlayingLine => {
					let player = get_player(state)?;

					let time;
					(time, state.lyrics.time_index_hint) = state
						.lyrics
						.time_index
						.find_seq(player.position(), state.lyrics.time_index_hint);

					if let Some(y) = time.line_num {
						state
							.cursor
							.set_y(y)
							.update_pos(state.lyrics.lyrics.line_widths())
							.update_scroll(
								Position::new(
									state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
									state.lyrics.lyrics.line_count(),
								),
								state.lyrics.screen_size,
								state.config.settings.scrolloff,
							);
					}
				}
				Action::SeekRelative(relative_pos) => {
					let pos = state.audio.seek_relative(relative_pos)?;
					if let Some(time) = pos {
						(_, state.lyrics.time_index_hint) =
							state.lyrics.time_index.find_random(time);
					}
				}
				Action::SeekBackwards(seconds) => {
					let player = get_player(state)?;
					let pos = player.position();
					player.seek(pos - min(Duration::from_secs_f32(seconds), pos))?;
				}
				Action::SeekForwards(seconds) => {
					let player = get_player(state)?;
					player.seek(player.position() + Duration::from_secs_f32(seconds))?;
				}
				Action::SeekToCursor => {
					let player = get_player(state)?;

					if let Some(timestamp) = state
						.lyrics
						.lyrics
						.time_at_cursor(state.cursor.pos().x, state.cursor.pos().y)
					{
						player.seek(timestamp.time() + Duration::from_millis(1))?;
						(_, state.lyrics.time_index_hint) =
							state.lyrics.time_index.find_random(timestamp.time());
					}
				}
				Action::SeekToCursorLine => {
					let player = get_player(state)?;

					if let Some(timestamp) = state.lyrics.lyrics.time_at_line(state.cursor.pos().y)
					{
						player.seek(timestamp.time() + Duration::from_millis(1))?;
						(_, state.lyrics.time_index_hint) =
							state.lyrics.time_index.find_random(timestamp.time());
					}
				}
				Action::TogglePause => {
					let player = get_player(state)?;
					player.set_paused(!player.is_paused());
				}
				Action::ChangeVolume(pct) => {
					let player = get_player(state)?;
					let volume = (player.volume() * 100. + 0.5) as i16 + pct;
					player.set_volume(min(max(volume, 0), 100) as f32 / 100.);
				}
				Action::ChangeSpeed(pct) => {
					let player = get_player(state)?;
					let speed = (player.speed() * 100. + 0.5) as i16 + pct;
					player.set_speed(min(max(speed, 50), 200) as f32 / 100.);
				}
				Action::ResetSpeed => {
					let player = get_player(state)?;
					player.set_speed(1.);
				}
				Action::Undo => {
					state.lyrics.undo()?;
				}
				Action::Redo => {
					state.lyrics.redo()?;
				}
				Action::SyncTimestamp => {
					let player = get_player(state)?;
					state
						.lyrics
						.set_timestamp(state.cursor.pos(), Some(player.position()))?;
					state
						.cursor
						.set_y(state.cursor.pos().y + 1)
						.update_pos(state.lyrics.lyrics.line_widths())
						.update_scroll(
							Position::new(
								state.lyrics.lyrics.line_widths().max().unwrap_or_default(),
								state.lyrics.lyrics.line_count(),
							),
							state.lyrics.screen_size,
							state.config.settings.scrolloff,
						);
					state.cursor.set_y(state.cursor.pos().y);
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
				}
				Action::OpenInEditor => {
					state.open_in_editor()?;
				}
				_ => return Ok(false),
			}
			Ok(true)
		} else {
			Ok(false)
		}
	}
}

fn get_player(state: &AppState) -> eyre::Result<&AudioPlayer> {
	state
		.audio
		.audio_player
		.as_ref()
		.ok_or_eyre("No audio playing")
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
