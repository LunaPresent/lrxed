use std::iter;

use ratatui::{
	layout::{Constraint, Flex, Layout},
	widgets::{Clear, Paragraph, StatefulWidget, Widget},
};

use crate::{state::AppState, tui::widgets::ToastWidget};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ToastsOverlay;

impl StatefulWidget for ToastsOverlay {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		const WIDTH: u16 = 50;
		const HEIGHT: u16 = 6;

		state
			.toasts
			.cull(state.config.settings.notification_timeout);

		let [area] = Layout::horizontal([Constraint::Length(WIDTH)])
			.horizontal_margin(4)
			.vertical_margin(1)
			.flex(Flex::End)
			.areas(area);

		let max_on_screen = (area.height / HEIGHT).max(1);

		let areas = Layout::vertical(Constraint::from_lengths(iter::repeat_n(
			HEIGHT,
			state.toasts.iter().count().min(max_on_screen as usize),
		)))
		.split(area);

		for (toast, &area) in state.toasts.iter().zip(areas.iter()) {
			Clear.render(area, buf);

			let toast = ToastWidget::new(Paragraph::new(toast.text.as_str()))
				.border_style(state.config.theme.border_err);
			toast.render(area, buf);
		}
	}
}
