use core::num;

use thiserror::Error;

use super::Command;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum CommandIntoAppEventError {
	#[error("malformed command: '{0}'")]
	BadCommand(Command),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseCommandError {
	#[error("invalid command: '{0}'")]
	InvalidCommand(String),
	#[error("failed to parse parameter for command '{cmd}': {inner}")]
	InvalidParameter {
		cmd: String,
		inner: ParseCommandParameterError,
	},
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseCommandParameterError {
	#[error("no parameters allowed for this command")]
	NoAllowedParameters,
	#[error("failed to parse parameter: '{0}'")]
	InvalidParameter(String),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParsePlaybackParameterError {
	#[error("invalid playback command keyword: '{0}'")]
	InvalidKeyword(String),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseIntegerParameterError {
	#[error(transparent)]
	InvalidInteger(#[from] num::ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParsePercentageParameterError {
	#[error(transparent)]
	InvalidNumber(#[from] num::ParseFloatError),
	#[error("missing percent sign")]
	MissingPercentSign,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ParseTimeParameterError {
	#[error("invalid time format: {0}")]
	InvalidFormat(String),
	#[error(transparent)]
	InvalidInteger(#[from] num::ParseIntError),
	#[error(transparent)]
	InvalidNumber(#[from] num::ParseFloatError),
}

#[derive(Debug, Clone, Copy, PartialEq, Error)]
#[error("failed to parse polarity from character '{0}'")]
pub struct ParseParameterPolarityError(pub char);
