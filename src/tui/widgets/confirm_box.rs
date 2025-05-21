use std::cmp;

use ratatui::{
	layout::{Alignment, Constraint, Flex, Layout, Position},
	text::Span,
	widgets::{Block, Padding, StatefulWidget, Widget},
};

use crate::state::{AppState, ConfirmBoxAction};

const PADDING: u16 = 1;
const BUTTON_PADDING: u16 = 1;
const BUTTON_SPACING: u16 = 1;
const TEXT_YES: &str = "Yes";
const TEXT_NO: &str = "No";
const TEXT_CANCEL: &str = "Cancel";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmBox<'a> {
	pub title: &'a str,
	pub prompt: &'a str,
}

impl<'a> ConfirmBox<'a> {
	pub fn size_required(self) -> Position {
		Position::new(
			cmp::max(
				[TEXT_YES, TEXT_NO, TEXT_CANCEL]
					.iter()
					.fold(0, |len, text| {
						len + text.len() as u16 + 2 * BUTTON_PADDING + BUTTON_SPACING
					}) - BUTTON_SPACING,
				cmp::max(self.prompt.len(), self.title.len()) as u16,
			) + 2 * PADDING
				+ 2,
			3 + 2,
		)
	}
}

impl<'a> StatefulWidget for ConfirmBox<'a> {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		let block = Block::bordered()
			.padding(Padding::symmetric(PADDING, 0))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.border_style(state.config.theme.confirm_box)
			.title_alignment(Alignment::Center)
			.title(self.title);
		let inner = block.inner(area);
		block.render(area, buf);

		let [prompt_area, button_area] =
			Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
				.flex(Flex::SpaceBetween)
				.areas(inner);

		let [prompt_area] = Layout::horizontal([Constraint::Length(self.prompt.len() as u16)])
			.flex(Flex::Center)
			.areas(prompt_area);
		Span::from(self.prompt).render(prompt_area, buf);

		let mut current_action = ConfirmBoxAction::default();
		let button_texts = [TEXT_YES, TEXT_NO, TEXT_CANCEL];
		let button_areas = Layout::horizontal(
			button_texts.map(|text| Constraint::Length(text.len() as u16 + 2 * BUTTON_PADDING)),
		)
		.spacing(1)
		.flex(Flex::Center)
		.split(button_area);
		for (text, &area) in button_texts.into_iter().zip(button_areas.iter()) {
			let style = if state.modal.confirm_box_selected == current_action {
				state.config.theme.button_active
			} else {
				state.config.theme.button_inactive
			};
			let block = Block::new()
				.padding(Padding::symmetric(BUTTON_PADDING, 0))
				.style(style);
			let inner = block.inner(area);
			block.render(area, buf);

			Span::from(text).render(inner, buf);
			current_action = current_action.next();
		}
	}
}
