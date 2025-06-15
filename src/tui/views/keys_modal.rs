use ratatui::{
	layout::{Alignment, Constraint, Flex, Layout},
	widgets::{Block, BorderType, Clear, Padding, StatefulWidget, Widget},
};

use crate::{
	config::{Action, Context},
	state::AppState,
	tui::{input_handler::InputHandler, widgets::KeysWidget},
};

pub struct KeysModal;

impl InputHandler for KeysModal {
	type State = AppState;

	fn handle_input(
		self,
		key_chord: crate::config::KeyChord,
		state: &mut Self::State,
	) -> color_eyre::eyre::Result<bool> {
		let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::ScrollablePopup)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		else {
			return Ok(false);
		};

		match action {
			Action::MoveCursorY { amount } => {
				state.modal.popup_scroll = (state.modal.popup_scroll as i16
					+ amount.max(-(state.modal.popup_scroll as i16)))
					as u16;
			}
			Action::SetCursorY { y } => state.modal.popup_scroll = y,
			Action::Cancel => {
				state.active_modal = None;
			}
			_ => return Ok(false),
		};

		Ok(true)
	}
}

impl StatefulWidget for KeysModal {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		let [area] = Layout::horizontal([Constraint::Max(100)])
			.flex(Flex::Center)
			.areas(area);
		let [area] = Layout::vertical([Constraint::Percentage(80)])
			.flex(Flex::Center)
			.areas(area);

		Clear.render(area, buf);

		let block = Block::bordered()
			.padding(Padding::symmetric(1, 0))
			.border_type(BorderType::Rounded)
			.border_style(state.config.theme.border_info)
			.title_alignment(Alignment::Center)
			.title("Keys");
		let inner = block.inner(area);
		block.render(area, buf);

		KeysWidget.render(inner, buf, state);
	}
}
