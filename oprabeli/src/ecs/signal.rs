use bevy_ecs::resource::Resource;
use thiserror::Error;

use crate::app::AppControls;

#[derive(Debug, Resource)]
pub struct Signal {
	controls: AppControls,
}

#[derive(Debug, Error)]
pub enum SignalError {
	#[error("failed to send quit signal")]
	Quit,
	#[error("failed to send suspend signal")]
	Suspend,
}

impl Signal {
	pub(crate) fn new(controls: AppControls) -> Self {
		Self { controls }
	}

	pub fn quit(&mut self) -> Result<(), SignalError> {
		self.controls.quit().map_err(|_| SignalError::Quit)
	}

	pub fn suspend(&mut self) -> Result<(), SignalError> {
		self.controls.suspend().map_err(|_| SignalError::Suspend)
	}
}
