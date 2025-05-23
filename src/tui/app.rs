use std::time::Duration;

use color_eyre::{Result, eyre};
use crossterm::event::{Event, EventStream, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use tokio_stream::StreamExt;

use crate::{
	config::{Action, Context, KeyChord},
	state::AppState,
};

use super::{
	Modal, View,
	input_handler::InputHandler,
	views::{ConfirmQuitModal, EditorView, FileTreeView},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct App;

impl App {
	const FRAMES_PER_SECOND: f32 = 60.0;

	pub async fn run(self, mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
		let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
		let mut interval = tokio::time::interval(period);
		let mut events = EventStream::new();

		while !state.should_quit {
			tokio::select! {
				_ = interval.tick() => {
					if state.refresh_term {
						terminal.clear()?;
						state.refresh_term = false;
					}
					terminal.draw(|frame| self.draw(frame, state))?;
				},
				Some(Ok(mut event)) = events.next() => self.handle_event(&mut event, state),
			}
		}
		Ok(())
	}

	fn draw(self, frame: &mut Frame, state: &mut AppState) {
		frame.render_stateful_widget(self, frame.area(), state);
		frame.set_cursor_position(state.cursor.render_pos());
	}

	fn handle_event(self, event: &mut Event, state: &mut AppState) {
		let result = match event {
			Event::Key(key) if key.kind == KeyEventKind::Press => {
				if let KeyCode::Char(_) = key.code {
					key.modifiers = key.modifiers.difference(KeyModifiers::SHIFT);
				}
				self.handle_input(KeyChord::new(key.code, key.modifiers), state)
					.map(|_| ())
			}
			_ => Ok(()),
		};
		// TODO: show error without crashing the whole thing
		result.unwrap();
	}
}

impl InputHandler for App {
	type State = AppState;

	fn handle_input(self, key_chord: KeyChord, state: &mut Self::State) -> eyre::Result<bool> {
		let consumed = if let Some(modal) = state.active_modal {
			match modal {
				Modal::ConfirmQuit => ConfirmQuitModal.handle_input(key_chord, state),
			}?
		} else {
			match state.active_view {
				View::FileTree => FileTreeView.handle_input(key_chord, state),
				View::Editor => EditorView.handle_input(key_chord, state),
			}?
		};
		if !consumed {
			match state.config.keys.get_action(key_chord, Context::Global) {
				// global keys here
				Some(Action::Quit) => {
					if state.lyrics.changed {
						state.active_modal = Some(Modal::ConfirmQuit);
					} else {
						state.should_quit = true;
					}
					Ok(true)
				}
				_ => Ok(false),
			}
		} else {
			Ok(true)
		}
	}
}

impl StatefulWidget for App {
	type State = AppState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
	where
		Self: Sized,
	{
		match state.active_view {
			View::FileTree => FileTreeView.render(area, buf, state),
			View::Editor => EditorView.render(area, buf, state),
		};
		if let Some(modal) = state.active_modal {
			match modal {
				Modal::ConfirmQuit => ConfirmQuitModal.render(area, buf, state),
			};
		}
	}
}
