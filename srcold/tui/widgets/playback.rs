use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Flex, Layout, Rect},
	style::{Style, Stylize},
	text::{Line, Span},
	widgets::{Block, Borders, LineGauge, Padding, StatefulWidget, Widget},
};

use crate::state::AppState;

use super::volume::VolumeWidget;

const BAR_FULL_OFFSET: f64 = 0.03125;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaybackWidget;

impl StatefulWidget for PlaybackWidget {
	type State = AppState;

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
			.border_type(ratatui::widgets::BorderType::Rounded)
			.border_style(state.config.theme.accent);
		let inner = block.inner(area);
		block.render(area, buf);

		if let Some(ref player) = state.audio.audio_player {
			let [_info_area, progress_area, controls_area] = Layout::vertical([
				Constraint::Length(1),
				Constraint::Length(1),
				Constraint::Length(1),
			])
			.areas(inner);

			{
				let [button_area, bar_area, label_area] = Layout::horizontal([
					Constraint::Length(1),
					Constraint::Fill(1),
					Constraint::Length(20),
				])
				.spacing(1)
				.areas(progress_area);

				let button_symbol = if player.is_paused() { "󰐊" } else { "󰏤" };
				Span::from(button_symbol).render(button_area, buf);

				let mp = player.position().as_secs() / 60;
				let sp = player.position().as_secs() % 60;
				let cp = player.position().subsec_millis() / 10;
				let md = player.duration().as_secs() / 60;
				let sd = player.duration().as_secs() % 60;
				let cd = player.duration().subsec_millis() / 10;
				let bar = LineGauge::default()
					.ratio(
						(player.position().as_secs_f64()
							/ (player.duration().as_secs_f64() - BAR_FULL_OFFSET))
							.min(1.0),
					)
					.unfilled_style(state.config.theme.inactive)
					.filled_style(state.config.theme.accent)
					.label("");
				bar.render(bar_area, buf);

				let label = Line::from(vec![
					Span::from(format!(" {mp:0>2}:{sp:0>2}.{cp:0>2}")),
					Span::from(format!(" / {md:0>2}:{sd:0>2}.{cd:0>2}"))
						.style(state.config.theme.text_secondary),
				])
				.style(Style::new().bold());
				label.render(label_area, buf);
			}

			{
				let [volume_area] = Layout::horizontal([Constraint::Length(17)])
					.spacing(1)
					.flex(Flex::Center)
					.areas(controls_area);

				VolumeWidget::new(player.volume())
					.unfilled_style(state.config.theme.inactive)
					.filled_style(state.config.theme.accent)
					.render(volume_area, buf);
			}
		} else {
			let msg = Span::from("No audio source loaded");
			msg.render(inner, buf);
		}
	}
}
