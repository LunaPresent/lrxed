use crate::tui::{Cursor, Modal, View};

use super::{AudioState, Config, LyricsState, ModalState};

pub struct AppState {
	pub audio: AudioState,
	pub lyrics: LyricsState,
	pub modal: ModalState,
	pub cursor: Cursor,
	pub config: Config,
	pub active_view: View,
	pub active_modal: Option<Modal>,
	pub should_quit: bool,
}

impl AppState {
	pub fn new(initial_view: View) -> Self {
		Self {
			audio: Default::default(),
			lyrics: Default::default(),
			modal: Default::default(),
			cursor: Default::default(),
			config: Default::default(),
			active_view: initial_view,
			active_modal: None,
			should_quit: false,
		}
	}
}
