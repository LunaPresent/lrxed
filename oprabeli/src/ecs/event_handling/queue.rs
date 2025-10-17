use std::collections::VecDeque;

use bevy_ecs::resource::Resource;

use crate::{ecs::DynEventDispatch, event::DispatchMethod};

#[derive(Resource, Default)]
pub struct EventQueue(VecDeque<DynEventDispatch>);

impl EventQueue {
	pub fn send<T>(&mut self, dispatch: DispatchMethod, event: T)
	where
		T: Send + Sync + 'static,
	{
		self.0.push_back(DynEventDispatch::new(dispatch, event));
	}

	pub(crate) fn next(&mut self) -> Option<DynEventDispatch> {
		self.0.pop_front()
	}
}
