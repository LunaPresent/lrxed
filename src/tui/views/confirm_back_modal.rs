use crate::{state::AppState, tui::views::confirm_modal::ConfirmModal};
use color_eyre::eyre;
use ratatui::widgets::StatefulWidget;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfirmBackModal;

impl ConfirmModal for ConfirmBackModal {
	const TITLE: &str = "Go Back?";
	const PROMPT: &str = "Save changes before returning to file browser?";

	fn exec_yes(self, state: &mut AppState) -> eyre::Result<()> {
		state
			.song
			.write_to_file(state.config.settings.replace_txt_file_on_save)?;

		state
			.file_browser
			.update_selected_song(state.song.song.clone());

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
