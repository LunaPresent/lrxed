use core::convert::Infallible;

pub(crate) trait IntoResult<T> {
	type Error;

	fn into_result(self) -> Result<T, Self::Error>;
}

impl<T> IntoResult<T> for T {
	type Error = Infallible;

	fn into_result(self) -> Result<T, Self::Error> {
		Ok(self)
	}
}

impl<T, E> IntoResult<T> for Result<T, E> {
	type Error = E;

	fn into_result(self) -> Result<T, Self::Error> {
		self
	}
}
