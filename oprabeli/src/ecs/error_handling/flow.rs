/// Signify whether an error handling system should catch or propagate an error
///
/// This value must be returned from an error handling system to tell the
/// dispatcher how it should handle the error after running the system with it
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorFlow {
	/// Signal to the system dispatcher that the error was handled
	Catch,
	/// Signal to the system dispatcher to bubble the error up the hierarchy,
	/// calling the parent entity's error handling system with the same error.
	/// If an error is propagated all the way through, the program will exit
	/// and emit this error
	Propagate,
	/// Signal to the system dispatcher to immediately fail error handling,
	/// causing the program to exit and emit this error
	Explode,
}
