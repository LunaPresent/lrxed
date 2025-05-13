use color_eyre::eyre;
use ratatui::widgets::StatefulWidget;

use crate::{state::AppState, tui::input_handler::InputHandler};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTreeView;

impl InputHandler for FileTreeView {
	type State = AppState;

	fn handle_input(
		&mut self,
		key: &crossterm::event::KeyEvent,
		state: &mut Self::State,
	) -> eyre::Result<bool> {
		Ok(false)
	}
}

impl StatefulWidget for FileTreeView {
	type State = AppState;

	fn render(
		self,
		area: ratatui::prelude::Rect,
		buf: &mut ratatui::prelude::Buffer,
		state: &mut Self::State,
	) {
		todo!()
	}
}
