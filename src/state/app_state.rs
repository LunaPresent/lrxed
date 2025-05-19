use crate::tui::{Cursor, View};

use super::{AudioState, Config, LyricsState};

pub struct AppState {
	pub audio: AudioState,
	pub lyrics: LyricsState,
	pub cursor: Cursor,
	pub config: Config,
	pub active_view: View,
	pub should_quit: bool,
}

impl AppState {
	pub fn new(initial_view: View) -> Self {
		Self {
			audio: Default::default(),
			lyrics: Default::default(),
			cursor: Default::default(),
			config: Default::default(),
			active_view: initial_view,
			should_quit: false,
		}
	}
}
