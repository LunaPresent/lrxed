use crate::{state::AppState, tui::views::confirm_modal::ConfirmModal};
use color_eyre::eyre;
use ratatui::widgets::StatefulWidget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmBackModal;

impl ConfirmModal for ConfirmBackModal {
	const TITLE: &str = "Go Back?";
	const PROMPT: &str = "Go back to file browser without saving changes?";

	fn exec_yes(self, state: &mut AppState) -> eyre::Result<()> {
		state
			.lyrics
			.write_to_file(state.config.settings.replace_txt_file_on_save)?;

		state.should_go_back = true;

		Ok(())
	}

	fn exec_no(self, state: &mut AppState) -> eyre::Result<()> {
		state.should_go_back = true;
		Ok(())
	}
}

impl StatefulWidget for ConfirmBackModal {
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
