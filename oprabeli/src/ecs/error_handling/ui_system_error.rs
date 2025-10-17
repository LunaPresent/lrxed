use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;

use super::error::ErrorHandleError;
use super::system_runner::RunErrorSystems;

const MAX_RECURSE: u16 = 1024;

pub(crate) trait UiSystemResultExt {
	type Value;

	fn handle(
		self,
		target: Entity,
		world: &mut World,
	) -> Result<Option<Self::Value>, ErrorHandleError>;
}

impl<T> UiSystemResultExt for Result<T, UiSystemError> {
	type Value = T;

	fn handle(
		self,
		target: Entity,
		world: &mut World,
	) -> Result<Option<Self::Value>, ErrorHandleError> {
		let handled = self.map_err(|e| e.handle(target, world));
		match handled {
			Ok(value) => Ok(Some(value)),
			Err(Ok(())) => Ok(None),
			Err(Err(e)) => Err(e),
		}
	}
}

pub(crate) trait UiSystemResultInternalExt {
	type Value;

	fn handle_internal(
		self,
		target: Entity,
		world: &mut World,
		recurse_depth: u16,
	) -> Result<Option<Self::Value>, ErrorHandleError>;
}

impl<T> UiSystemResultInternalExt for Result<T, UiSystemError> {
	type Value = T;

	fn handle_internal(
		self,
		target: Entity,
		world: &mut World,
		recurse_depth: u16,
	) -> Result<Option<Self::Value>, ErrorHandleError> {
		let handled = self.map_err(|e| e.handle_internal(target, world, recurse_depth));
		match handled {
			Ok(value) => Ok(Some(value)),
			Err(Ok(())) => Ok(None),
			Err(Err(e)) => Err(e),
		}
	}
}

#[derive(Debug)]
pub(crate) struct UiSystemError {
	pub error_system_runner: Box<dyn RunErrorSystems>,
}

impl UiSystemError {
	pub fn handle(self, target: Entity, world: &mut World) -> Result<(), ErrorHandleError> {
		self.handle_internal(target, world, 0)
	}

	fn handle_internal(
		mut self,
		target: Entity,
		world: &mut World,
		recurse_depth: u16,
	) -> Result<(), ErrorHandleError> {
		if recurse_depth >= MAX_RECURSE {
			Err(ErrorHandleError::MaxRecursion)?;
		}
		self.error_system_runner
			.run_error_systems(target, world, recurse_depth)?;
		Ok(())
	}
}
