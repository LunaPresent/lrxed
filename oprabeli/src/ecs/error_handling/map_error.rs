use super::system_runner::ErrorSystemRunner;
use super::ui_system_error::UiSystemError;
use crate::ecs::into_result::IntoResult;

pub(crate) fn map_system_error<R, T>(result: R) -> Result<T, UiSystemError>
where
	R: IntoResult<T, Error: Into<eyre::Report> + core::fmt::Debug + 'static>,
{
	result.into_result().map_err(|e| UiSystemError {
		error_system_runner: Box::new(ErrorSystemRunner::new(e)),
	})
}
