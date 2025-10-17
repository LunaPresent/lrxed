use thiserror::Error;

// TODO: documentation
#[derive(Debug, Error)]
pub enum ViewportError {
	#[error("viewport offset out of bounds")]
	OffsetOutOfBounds,
	#[error("viewport size too small for target area")]
	TooSmall,
	#[error("viewport component no longer exists")]
	Missing,
}
