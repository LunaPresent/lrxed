use ratatui::prelude::*;
use std::iter::repeat_n;

use crate::{
	lyrics::{LyricLine, Lyrics},
	state::Config,
};

pub struct LyricsPreviewWidget<'l, 'c> {
	lyrics: &'l Lyrics,
	config: &'c Config,
}

impl<'l, 'c> LyricsPreviewWidget<'l, 'c> {
	pub fn new(lyrics: &'l Lyrics, config: &'c Config) -> Self {
		Self { lyrics, config }
	}
}

impl Widget for LyricsPreviewWidget<'_, '_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let rows = area.height.min(self.lyrics.line_count()) as usize;
		let layout = Layout::vertical(repeat_n(Constraint::Length(1), rows)).split(area);

		for (line, &row) in self.lyrics.lines().iter().take(rows).zip(layout.iter()) {
			let lyric_line_preview = LyricLinePreviewWidget {
				line,
				timestamp_style: self.config.theme.accent,
				show_timestamps: self.lyrics.sync_percentage() > 0,
			};

			lyric_line_preview.render(row, buf);
		}
	}
}

struct LyricLinePreviewWidget<'a> {
	line: &'a LyricLine,
	timestamp_style: Style,
	show_timestamps: bool,
}

impl Widget for LyricLinePreviewWidget<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		if self.show_timestamps {
			let constraints = [Constraint::Length(8), Constraint::Fill(1)];
			let [time_area, text_area] = Layout::horizontal(constraints).spacing(1).areas(area);

			Text::styled(self.line.timestamp_text(), self.timestamp_style).render(time_area, buf);
			Text::from(self.line.text()).render(text_area, buf)
		} else {
			Text::from(self.line.text()).render(area, buf);
		}
	}
}
