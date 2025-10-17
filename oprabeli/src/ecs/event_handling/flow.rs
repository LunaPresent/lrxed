/// Signify whether an update system should consume or propagate an event
///
/// This value must be returned from an update system to tell the dispatcher
/// how it should handle the event after running the system with it
/// The value is ignored if the event was dispatched as [`Dispatch::Broadcast`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventFlow {
	/// Signal to the system dispatcher to stop propagating the event
	Consume,
	/// Signal to the system dispatcher to bubble the event up the hierarchy,
	/// calling the parent entity's update system with the same event
	Propagate,
}
