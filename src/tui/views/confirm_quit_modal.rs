use color_eyre::eyre;
use ratatui::{
	layout::{Constraint, Flex, Layout},
	widgets::{Clear, StatefulWidget, Widget},
};

use crate::{
	config::{Action, Context, KeyChord},
	state::{AppState, ConfirmBoxAction},
	tui::{input_handler::InputHandler, widgets::ConfirmBox},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmQuitModal;

impl ConfirmQuitModal {
	pub fn exec_yes(self, state: &mut AppState) -> eyre::Result<()> {
		state.lyrics.write_to_file()?;
		state.should_quit = true;
		Ok(())
	}

	pub fn exec_no(self, state: &mut AppState) -> eyre::Result<()> {
		state.should_quit = true;
		Ok(())
	}

	pub fn exec_cancel(self, state: &mut AppState) -> eyre::Result<()> {
		state.active_modal = None;
		Ok(())
	}
}

impl InputHandler for ConfirmQuitModal {
	type State = AppState;

	fn handle_input(self, key_chord: KeyChord, state: &mut AppState) -> eyre::Result<bool> {
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

impl StatefulWidget for ConfirmQuitModal {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		let confirm_box = ConfirmBox {
			title: "Confirm Quit",
			prompt: "Save changes before quitting?",
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
