use ratatui::{
	prelude::{Buffer, Rect},
	widgets::StatefulWidget,
};

use crate::state::AppState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectoryWidget;

impl StatefulWidget for DirectoryWidget {
	type State = AppState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		todo!()
	}
}
