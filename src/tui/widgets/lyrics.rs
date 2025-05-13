use std::{cmp, iter};

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::Style,
	symbols,
	text::Span,
	widgets::{Block, StatefulWidget, Widget},
};

use crate::{lyrics::TimeIndexEntry, state::AppState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LyricsWidget;

impl StatefulWidget for LyricsWidget {
	type State = AppState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
	where
		Self: Sized,
	{
		state.lyrics.bufsize.y = area.height as usize;
		let rows = cmp::min(
			area.height as usize,
			state.lyrics.lyrics.line_count() - state.lyrics.scroll_y,
		);
		let layout = Layout::vertical(
			iter::repeat_n(Constraint::Length(1), rows).chain(iter::once(Constraint::Fill(1))),
		);

		let areas = layout.split(area);
		let (_fill, line_areas) = areas.split_last().unwrap();
		let lyrics_lines = state
			.lyrics
			.lyrics
			.lines()
			.enumerate()
			.skip(state.lyrics.scroll_y)
			.take(rows);

		let mut current_lyric_line = TimeIndexEntry::default();
		if let Some(player) = &state.audio.audio_player {
			(current_lyric_line, state.lyrics.time_index_hint) = state
				.lyrics
				.time_index
				.find_seq(player.position(), state.lyrics.time_index_hint);
		}

		for ((line_num, lyric_line), &line_area) in lyrics_lines.zip(line_areas) {
			let select_style = if line_num == state.lyrics.cursor.y {
				state.config.theme.cursorline
			} else {
				Style::default()
			};
			Block::new().style(select_style).render(line_area, buf);
			let line_layout = Layout::horizontal([
				Constraint::Length(8),
				Constraint::Length(1),
				Constraint::Length(2),
				Constraint::Fill(1),
			]);

			let is_current_lyric = Some(line_num) == current_lyric_line.line_num;
			let lyrics_style = if is_current_lyric {
				state.config.theme.current_line
			} else {
				Style::default()
			};

			let [time_area, border_area, mark_area, text_area] = line_layout.areas(line_area);
			Span::styled(lyric_line.timestamp_text(), lyrics_style).render(time_area, buf);
			Span::styled(symbols::line::THICK_VERTICAL, lyrics_style).render(border_area, buf);
			if is_current_lyric {
				Span::styled("𝅘𝅥𝅮", lyrics_style).render(mark_area, buf);
			}
			Span::styled(lyric_line.text(), lyrics_style).render(text_area, buf);
		}
	}
}
