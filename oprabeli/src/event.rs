mod error;
mod system;

pub use error::EventError;
pub use system::EventSystem;

use std::time::Duration;

use bevy_ecs::entity::Entity;
use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug)]
pub struct EventDispatch<T> {
	pub dispatch: DispatchMethod,
	pub event: T,
}

impl<T> EventDispatch<T> {
	pub fn new(dispatch: DispatchMethod, event: T) -> Self {
		Self { dispatch, event }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchMethod {
	Input,
	Broadcast,
	Cursor {
		x: u16,
		y: u16,
		action: CursorAction,
	},
	Target(Entity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorAction {
	Retain,
	Engage,
	Release,
}

#[derive(Debug, Clone)]
pub enum SystemEvent {
	Tick(Duration),
	FocusGained,
	FocusLost,
	Key(KeyEvent),
	Mouse(MouseEvent),
	Paste(String),
	Resize { width: u16, height: u16 },
}
