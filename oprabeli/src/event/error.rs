use thiserror::Error;
use tokio::sync::mpsc::error::SendError;
use tokio::task::JoinError;

use super::{EventDispatch, SystemEvent};

#[derive(Debug, Error)]
pub enum EventError {
	#[error("event channel disconnected")]
	Disconnected,
	#[error("task is already running")]
	AlreadyRunning,
	#[error("task is already stopped")]
	AlreadyStopped,
	#[error("failed to join thread")]
	JoinError(#[from] JoinError),
	#[error("failed to send event")]
	SendError(#[from] SendError<EventDispatch<SystemEvent>>),
}
