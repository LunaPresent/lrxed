use thiserror::Error;
use tokio::sync::mpsc;

#[derive(Debug)]
pub(crate) enum SignalType {
	Quit,
	Suspend,
}

#[derive(Debug, Error)]
pub(crate) enum AppControlsError {
	#[error(transparent)]
	Stop(#[from] mpsc::error::SendError<()>),
	#[error(transparent)]
	Signal(#[from] mpsc::error::SendError<SignalType>),
}

#[derive(Debug, Clone)]
pub(crate) struct AppControls {
	stop: mpsc::UnboundedSender<()>,
	signal: mpsc::UnboundedSender<SignalType>,
}

#[derive(Debug)]
pub(crate) struct AppSignalReceiver {
	pub stop: mpsc::UnboundedReceiver<()>,
	pub signal: mpsc::UnboundedReceiver<SignalType>,
}

impl AppControls {
	pub fn new() -> (AppControls, AppSignalReceiver) {
		let stop = mpsc::unbounded_channel();
		let signal = mpsc::unbounded_channel();

		(
			AppControls {
				stop: stop.0,
				signal: signal.0,
			},
			AppSignalReceiver {
				stop: stop.1,
				signal: signal.1,
			},
		)
	}

	pub fn quit(&mut self) -> Result<(), AppControlsError> {
		self.stop.send(())?;
		self.signal.send(SignalType::Quit)?;
		Ok(())
	}

	pub fn suspend(&mut self) -> Result<(), AppControlsError> {
		self.stop.send(())?;
		self.signal.send(SignalType::Suspend)?;
		Ok(())
	}
}
