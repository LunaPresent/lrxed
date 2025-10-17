use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum KeyChordParseError {
	#[error("empty key chord string")]
	EmptyString,
	#[error("invalid modifier identifier")]
	InvalidModifier,
	#[error("duplicate key modifier")]
	DuplicateModifier,
	#[error("invalid key identifier")]
	InvalidKey,
	#[error("key chord has no closing tag")]
	UnclosedTag,
}
