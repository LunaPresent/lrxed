use std::{cmp, iter};

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	style::Style,
	symbols,
	text::Span,
	widgets::{Block, StatefulWidget, Widget},
};

use crate::state::AppState;

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
			state.lyrics.lyrics.lines().len() - state.lyrics.scroll_y,
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
			.iter()
			.enumerate()
			.skip(state.lyrics.scroll_y)
			.take(rows);
		for ((line_num, lyric_line), &line_area) in lyrics_lines.zip(line_areas) {
			let style = if line_num == state.lyrics.cursor.y {
				state.config.theme.cursorline
			} else {
				Style::default()
			};
			Block::new().style(style).render(line_area, buf);
			let line_layout = Layout::horizontal([
				Constraint::Length(8),
				Constraint::Length(2),
				Constraint::Fill(1),
			]);

			let [time_area, border_area, text_area] = line_layout.areas(line_area);
			Span::styled(lyric_line.timestamp_text(), style).render(time_area, buf);
			Span::styled(symbols::line::THICK_VERTICAL, style).render(border_area, buf);
			Span::styled(lyric_line.text(), style).render(text_area, buf);
		}
	}
}
