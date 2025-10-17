use bevy_ecs::resource::Resource;
use tokio::sync::mpsc;

use crate::{ecs::DynEventDispatch, event::DispatchMethod};

#[derive(Debug, Resource, Clone)]
pub struct AsyncEventQueue {
	sender: mpsc::UnboundedSender<DynEventDispatch>,
}

impl AsyncEventQueue {
	pub(crate) fn new(sender: mpsc::UnboundedSender<DynEventDispatch>) -> Self {
		Self { sender }
	}

	pub fn send<T>(&mut self, dispatch: DispatchMethod, event: T)
	where
		T: Send + Sync + 'static,
	{
		let _ = self.sender.send(DynEventDispatch::new(dispatch, event));
	}
}
