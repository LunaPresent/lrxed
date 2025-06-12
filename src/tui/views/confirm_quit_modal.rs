use crate::{state::AppState, tui::views::confirm_modal::ConfirmModal};
use color_eyre::eyre;
use ratatui::widgets::StatefulWidget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmQuitModal;

impl ConfirmModal for ConfirmQuitModal {
	const TITLE: &str = "Confirm Quit";
	const PROMPT: &str = "Save changes before quitting?";

	fn exec_yes(self, state: &mut AppState) -> eyre::Result<()> {
		state.lyrics.write_to_file()?;
		state.should_quit = true;
		Ok(())
	}

	fn exec_no(self, state: &mut AppState) -> eyre::Result<()> {
		state.should_quit = true;
		Ok(())
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
		ConfirmModal::render(&self, area, buf, state);
	}
}
