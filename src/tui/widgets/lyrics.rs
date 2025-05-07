use std::{cmp, iter};

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	symbols,
	text::Span,
	widgets::{StatefulWidget, Widget},
};

use crate::state::LyricsState;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LyricsWidget;

impl StatefulWidget for LyricsWidget {
	type State = LyricsState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
	where
		Self: Sized,
	{
		let rows = cmp::min(
			area.height as usize,
			state.lyrics.lines().len() - state.scroll_y,
		);
		let layout = Layout::vertical(
			iter::repeat_n(Constraint::Length(1), rows).chain(iter::once(Constraint::Fill(1))),
		);

		let areas = layout.split(area);
		let (_fill, line_areas) = areas.split_last().unwrap();
		let lyrics_lines = state.lyrics.lines().iter().skip(state.scroll_y).take(rows);
		for (lyrics_line, &line_area) in lyrics_lines.zip(line_areas) {
			let line_layout = Layout::horizontal([
				Constraint::Length(8),
				Constraint::Length(2),
				Constraint::Fill(1),
			]);

			let [time_area, border_area, text_area] = line_layout.areas(line_area);
			Span::from(lyrics_line.timestamp_text()).render(time_area, buf);
			Span::from(symbols::line::THICK_VERTICAL).render(border_area, buf);
			Span::from(lyrics_line.text()).render(text_area, buf);
		}
	}
}
