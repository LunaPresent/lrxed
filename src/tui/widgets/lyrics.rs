use std::{cmp, iter};

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Position, Rect},
	style::Style,
	symbols,
	text::Span,
	widgets::{Block, StatefulWidget, Widget},
};
use unicode_width::UnicodeWidthChar;

use crate::{lyrics::TimeIndexEntry, state::AppState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LyricsWidget;

impl StatefulWidget for LyricsWidget {
	type State = AppState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		let scroll_y = state
			.cursor
			.scroll()
			.y
			.min(state.song.song.lyrics.line_count().max(1) - 1);
		let rows = cmp::min(area.height, state.song.song.lyrics.line_count() - scroll_y) as usize;

		let layout = Layout::vertical(
			iter::repeat_n(Constraint::Length(1), rows).chain(iter::once(Constraint::Fill(1))),
		);

		let areas = layout.split(area);
		let (_fill, line_areas) = areas.split_last().unwrap();

		let lyrics_lines = state
			.song
			.song
			.lyrics
			.lines()
			.iter()
			.enumerate()
			.skip(scroll_y as usize)
			.take(rows);

		let line_layout = Layout::horizontal([
			Constraint::Length(8),
			Constraint::Length(1),
			Constraint::Length(2),
			Constraint::Fill(1),
		]);

		if let Some(&line_area) = line_areas.get((state.cursor.pos().y - scroll_y) as usize) {
			Block::new()
				.style(state.config.theme.cursorline)
				.render(line_area, buf);
		}

		let [_, _, _, text_area] = line_layout.areas(area);
		state.cursor.set_render_origin(text_area.positions().next());
		state
			.cursor
			.set_screen_size(Position::new(text_area.width, text_area.height));

		let mut current_lyric_line = TimeIndexEntry::default();
		if let Some(player) = &state.audio.audio_player {
			(current_lyric_line, state.song.time_index_hint) = state
				.song
				.time_index
				.find_seq(player.position(), state.song.time_index_hint);
		}

		for ((line_num, lyric_line), &line_area) in lyrics_lines.zip(line_areas) {
			let is_current_lyric = Some(line_num as u16) == current_lyric_line.line_num;
			let lyrics_style = if is_current_lyric {
				state.config.theme.lyrics_line
			} else {
				Style::default()
			};

			let [time_area, border_area, mark_area, text_area] = line_layout.areas(line_area);
			Span::styled(
				lyric_line.timestamp_text(),
				state.config.theme.accent.patch(lyrics_style),
			)
			.render(time_area, buf);
			Span::styled(symbols::line::THICK_VERTICAL, Style::default()).render(border_area, buf);
			if is_current_lyric {
				Span::styled("𝅘𝅥𝅮", state.config.theme.accent.patch(lyrics_style))
					.render(mark_area, buf);
			}
			let (pos, idx) = lyric_line
				.text()
				.char_indices()
				.scan(0, |pos, (idx, c)| {
					let current_pos = *pos;
					*pos += c.width().unwrap_or_default() as u16;
					Some((current_pos, idx))
				})
				.find(|(pos, _)| *pos >= state.cursor.scroll().x)
				.unwrap_or((state.cursor.scroll().x, lyric_line.text().len()));
			let offset = pos - state.cursor.scroll().x;
			let [offset_area, text_area] =
				Layout::horizontal([Constraint::Length(offset), Constraint::Fill(1)])
					.areas(text_area);
			Span::styled(">>>>", state.config.theme.text_secondary).render(offset_area, buf);
			Span::styled(&lyric_line.text()[idx..], lyrics_style).render(text_area, buf);
		}
	}
}
