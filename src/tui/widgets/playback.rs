use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Layout, Rect},
	text::Span,
	widgets::{Block, Borders, Gauge, Padding, StatefulWidget, Widget},
};

use crate::state::AudioState;

#[derive(Default, Clone, Copy)]
pub struct PlaybackWidget;

impl StatefulWidget for PlaybackWidget {
	type State = AudioState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
	where
		Self: Sized,
	{
		let [area] = Layout::horizontal([Constraint::Fill(1)])
			.horizontal_margin(1)
			.areas(area);
		let block = Block::new()
			.padding(Padding::symmetric(2, 0))
			.borders(Borders::ALL)
			.border_type(ratatui::widgets::BorderType::Rounded);
		let inner = block.inner(area);
		block.render(area, buf);

		if let Some(ref player) = state.audio_player {
			// let guage = Gauge::default().use_unicode(true);
			// guage.render(inner, buf);
			let msg = Span::from(format!(
				"{}/{}, stopped: {}",
				player.position().as_secs_f32(),
				player.duration().unwrap().as_secs_f32(),
				player.is_stopped(),
			));
			msg.render(inner, buf);
		} else {
			let msg = Span::from("No audio source loaded");
			msg.render(inner, buf);
		}
	}
}
