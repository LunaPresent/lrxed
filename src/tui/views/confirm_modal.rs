use color_eyre::eyre;

use ratatui::{
	buffer::Buffer,
	layout::{Constraint, Flex, Layout, Rect},
	widgets::{Clear, StatefulWidget, Widget},
};

use crate::{
	config::{Action, Context},
	state::{AppState, ConfirmBoxAction},
	tui::{input_handler::InputHandler, widgets::ConfirmBox},
};

pub trait ConfirmModal: StatefulWidget<State = AppState> {
	const TITLE: &str;
	const PROMPT: &str;

	fn exec_yes(self, state: &mut AppState) -> eyre::Result<()>;
	fn exec_no(self, state: &mut AppState) -> eyre::Result<()>;

	fn exec_cancel(&self, state: &mut AppState) -> eyre::Result<()> {
		state.active_modal = None;
		Ok(())
	}

	fn render(&self, area: Rect, buf: &mut Buffer, state: &mut AppState) {
		let confirm_box = ConfirmBox {
			title: Self::TITLE,
			prompt: Self::PROMPT,
		};

		let size = confirm_box.size_required();

		let [area] = Layout::horizontal([Constraint::Length(size.x)])
			.flex(Flex::Center)
			.areas(area);
		let [_, area, _] = Layout::vertical([
			Constraint::Fill(1),
			Constraint::Length(size.y),
			Constraint::Fill(2),
		])
		.areas(area);

		Clear.render(area, buf);
		confirm_box.render(area, buf, state);
	}
}

impl<T: ConfirmModal> InputHandler for T {
	type State = AppState;

	fn handle_input(
		self,
		key_chord: crate::config::KeyChord,
		state: &mut Self::State,
	) -> eyre::Result<bool> {
		if let Some(action) = state
			.config
			.keys
			.get_action(key_chord, Context::ConfirmBox)
			.or(state.config.keys.get_action(key_chord, Context::Global))
		{
			match action {
				Action::MoveCursorX(1) => {
					state.modal.confirm_box_selected = state.modal.confirm_box_selected.next();
				}
				Action::MoveCursorX(-1) => {
					state.modal.confirm_box_selected = state.modal.confirm_box_selected.prev();
				}
				Action::Confirm => {
					match state.modal.confirm_box_selected {
						ConfirmBoxAction::Yes => self.exec_yes(state),
						ConfirmBoxAction::No => self.exec_no(state),
						ConfirmBoxAction::Cancel => self.exec_cancel(state),
					}?;

					state.active_modal = None;
				}
				Action::Yes => {
					self.exec_yes(state)?;
				}
				Action::No => {
					self.exec_no(state)?;
				}
				Action::Cancel => {
					self.exec_cancel(state)?;
				}
				_ => return Ok(false),
			};
			Ok(true)
		} else {
			Ok(false)
		}
	}
}
