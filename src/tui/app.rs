use std::time::Duration;

use color_eyre::{Result, eyre};
use crossterm::event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use tokio_stream::StreamExt;

use crate::{
	config::{Action, KeyChord},
	state::AppState,
};

use super::{
	input_handler::InputHandler,
	views::{EditorView, FileTreeView},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
	FileTree,
	Editor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct App {
	active_view: View,
	should_quit: bool,
}

impl App {
	const FRAMES_PER_SECOND: f32 = 60.0;

	pub fn new(initial_view: View) -> Self {
		Self {
			active_view: initial_view,
			should_quit: false,
		}
	}

	pub async fn run(&mut self, mut terminal: DefaultTerminal, state: &mut AppState) -> Result<()> {
		let period = Duration::from_secs_f32(1.0 / Self::FRAMES_PER_SECOND);
		let mut interval = tokio::time::interval(period);
		let mut events = EventStream::new();

		while !self.should_quit {
			tokio::select! {
				_ = interval.tick() => { terminal.draw(|frame| self.draw(frame, state))?; },
				Some(Ok(mut event)) = events.next() => self.handle_event(&mut event, state),
			}
		}
		Ok(())
	}

	fn draw(self, frame: &mut Frame, state: &mut AppState) {
		frame.render_stateful_widget(self, frame.area(), state);
	}

	fn handle_event(&mut self, event: &mut Event, state: &mut AppState) {
		let result = match event {
			Event::Key(key) if key.kind == KeyEventKind::Press => {
				if let KeyCode::Char(_) = key.code {
					key.modifiers = key.modifiers.difference(KeyModifiers::SHIFT);
				}
				if let Some(action) = state
					.config
					.keys
					.get_action(KeyChord::new(key.code, key.modifiers))
				{
					self.handle_input(action, state).map(|_| ())
				} else {
					Ok(())
				}
			}
			_ => Ok(()),
		};
		// TODO: show error without crashing the whole thing
		result.unwrap();
	}
}

impl InputHandler for App {
	type State = AppState;

	fn handle_input(&mut self, action: Action, state: &mut Self::State) -> eyre::Result<bool> {
		let consumed = match self.active_view {
			View::FileTree => FileTreeView.handle_input(action, state),
			View::Editor => EditorView.handle_input(action, state),
		}?;
		if !consumed {
			match action {
				// global keys here
				Action::Quit => {
					self.should_quit = true;
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
		match self.active_view {
			View::FileTree => FileTreeView.render(area, buf, state),
			View::Editor => EditorView.render(area, buf, state),
		};
	}
}
