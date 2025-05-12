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
			Action::CursorDown => {
				if state.lyrics.cursor.y < state.lyrics.lyrics.line_count() - 1 {
					state
						.lyrics
						.cursor_to(state.lyrics.cursor.y + 1, state.config.settings.scrolloff);
				}
				Ok(true)
			}
			Action::CursorUp => {
				if state.lyrics.cursor.y > 0 {
					state
						.lyrics
						.cursor_to(state.lyrics.cursor.y - 1, state.config.settings.scrolloff);
				}
				Ok(true)
			}
			Action::CursorTop => {
				state.lyrics.cursor_to(0, state.config.settings.scrolloff);
				Ok(true)
			}
			Action::CursorBottom => {
				state.lyrics.cursor_to(
					state.lyrics.lyrics.line_count() - 1,
					state.config.settings.scrolloff,
				);
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
			Action::TogglePause => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				player.set_paused(!player.is_paused());
				Ok(true)
			}
			Action::VolumeDown => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let volume = (player.volume() * 100. + 0.5) as i16 - 10;
				player.set_volume(max(volume, 0) as f32 / 100.);
				Ok(true)
			}
			Action::VolumeUp => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let volume = (player.volume() * 100. + 0.5) as i16 + 10;
				player.set_volume(min(volume, 100) as f32 / 100.);
				Ok(true)
			}
			Action::VolumeDownSlightly => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let volume = (player.volume() * 100. + 0.5) as i16 - 1;
				player.set_volume(max(volume, 0) as f32 / 100.);
				Ok(true)
			}
			Action::VolumeUpSlightly => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let volume = (player.volume() * 100. + 0.5) as i16 + 1;
				player.set_volume(min(volume, 100) as f32 / 100.);
				Ok(true)
			}
			Action::SpeedDown => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let speed = (player.speed() * 20. + 0.5) as i16 - 1;
				player.set_speed(max(speed, 10) as f32 / 20.);
				Ok(true)
			}
			Action::SpeedUp => {
				let player = state
					.audio
					.audio_player
					.as_ref()
					.ok_or(eyre::eyre!("No audio playing"))?;
				let speed = (player.speed() * 20. + 0.5) as i16 + 1;
				player.set_speed(min(speed, 40) as f32 / 20.);
				Ok(true)
			}
			Action::SpeedReset => {
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
