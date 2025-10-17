use bevy_ecs::entity::Entity;
use bevy_ecs::hierarchy::ChildOf;
use bevy_ecs::system::{In, Query};
use bevy_ecs::world::World;

use super::error::ErrorHandleError;
use super::flow::ErrorFlow;
use super::ui_system_error::UiSystemResultInternalExt as _;
use crate::ecs::ui_component::*;

struct EntityErrorInfo<E>
where
	E: 'static,
{
	entity: Entity,
	system: ErrorSystemId<E>,
}

pub(crate) trait RunErrorSystems: core::fmt::Debug {
	fn run_error_systems(
		&mut self,
		target: Entity,
		world: &mut World,
		recurse_depth: u16,
	) -> Result<(), ErrorHandleError>;
}

#[derive(Debug)]
pub(crate) struct ErrorSystemRunner<E> {
	error: Option<E>,
}

impl<E> RunErrorSystems for ErrorSystemRunner<E>
where
	E: core::fmt::Debug + Into<eyre::Report> + 'static,
{
	fn run_error_systems(
		&mut self,
		target: Entity,
		world: &mut World,
		recurse_depth: u16,
	) -> Result<(), ErrorHandleError> {
		let error = self
			.error
			.take()
			.expect("error should never be none because public interface forbids it");
		let targets = world
			.run_system_cached_with(Self::find_target_entities, target)
			.map_err(|e| ErrorHandleError::RegisteredSystemError(e.into()))?;
		let mut catch = false;
		let mut prev_entity = Entity::PLACEHOLDER;

		for target in targets {
			// make multiple systems on one entity order independent by firing all of them even if
			// one returned consume
			if catch && prev_entity != target.entity {
				break;
			}
			prev_entity = target.entity;

			let flow = world
				.run_system_with(
					target.system,
					ErrorContext {
						entity: target.entity,
						error: &error,
					},
				)
				.map_err(|e| ErrorHandleError::RegisteredSystemError(e.into()))?
				.handle_internal(target.entity, world, recurse_depth + 1)?;
			match flow {
				Some(ErrorFlow::Propagate) => (),
				Some(ErrorFlow::Explode) => {
					catch = false;
					break;
				}
				_ => catch = true,
			}
		}
		if catch {
			Ok(())
		} else {
			Err(ErrorHandleError::Unhandled(error.into()))
		}
	}
}

impl<E> ErrorSystemRunner<E> {
	pub fn new(error: E) -> Self {
		Self { error: Some(error) }
	}

	fn find_target_entities(
		In(target): In<Entity>,
		handles: Query<&ErrorSystemCollection<E>>,
		parents: Query<&ChildOf>,
	) -> Vec<EntityErrorInfo<E>> {
		let mut targets = Vec::default();
		Self::bubble_entities(target, &mut targets, handles, parents);
		targets
	}

	fn bubble_entities(
		head: Entity,
		targets: &mut Vec<EntityErrorInfo<E>>,
		systems: Query<&ErrorSystemCollection<E>>,
		parents: Query<&ChildOf>,
	) {
		if let Ok(systems) = systems.get(head) {
			for &system in systems.iter() {
				targets.push(EntityErrorInfo {
					entity: head,
					system,
				});
			}
		}
		if let Ok(parent) = parents.get(head) {
			Self::bubble_entities(parent.parent(), targets, systems, parents);
		}
	}
}
