use bevy_ecs::system::RunSystemError;
use derive_more::From;
use thiserror::Error;

#[derive(Debug, Error, From)]
pub enum BevyErrorWrapper {
	#[error("RunSystemError: {0}")]
	RunSystemError(RunSystemError),
}
