use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ErrorHandleError {
	#[error("reached max recursion limit for error handling systems")]
	MaxRecursion,
	#[error("failed to run internal error handling systems")]
	RegisteredSystemError(#[source] eyre::Report),
	#[error("unhandled error in UI system")]
	Unhandled(#[source] eyre::Report),
}
