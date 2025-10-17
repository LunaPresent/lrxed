use core::convert::Infallible;

pub trait ConvertAction {
	type Event;

	fn to_event(&self) -> Self::Event;
}

pub trait TryConvertAction {
	type Event;
	type Error;

	fn try_to_event(&self) -> Result<Self::Event, Self::Error>;
}

impl<T> TryConvertAction for T
where
	T: ConvertAction,
{
	type Event = T::Event;
	type Error = Infallible;

	fn try_to_event(&self) -> Result<Self::Event, Self::Error> {
		Ok(self.to_event())
	}
}
