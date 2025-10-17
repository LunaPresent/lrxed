use core::marker::PhantomData;
use core::mem;

use bevy_ecs::system::{IntoSystem, SystemId};
use bevy_ecs::world::{EntityWorldMut, World};

use super::UiSystemContext;
use crate::ecs::error_handling::{UiSystemError, map_system_error};
use crate::ecs::into_result::IntoResult;

// TODO: documentation
pub struct UiSystem {
	boxed_system: Box<dyn GenericSystem + Sync + Send>,
}

impl UiSystem {
	// TODO: documentation
	#[allow(private_bounds)]
	pub fn new<C, R, M, S>(system: S) -> Self
	where
		C: UiSystemContext + 'static,
		R: IntoResult<C::Result, Error: Into<eyre::Report> + core::fmt::Debug> + 'static,
		M: 'static,
		S: IntoSystem<C, R, M> + Sync + Send + Clone + 'static,
	{
		Self {
			boxed_system: Box::new(GenericSystemImpl {
				registrar: CachedSystemRegistrar::System(system, PhantomData),
			}),
		}
	}

	pub(crate) fn register(&mut self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.register(entity_world);
	}

	pub(crate) fn unregister(&mut self, entity_world: &mut EntityWorldMut) {
		self.boxed_system.unregister(entity_world);
	}
}

trait GenericSystem {
	fn register(&mut self, entity_world: &mut EntityWorldMut);
	fn unregister(&mut self, entity_world: &mut EntityWorldMut);
}

struct GenericSystemImpl<C, R, M, S>
where
	C: UiSystemContext,
{
	registrar: CachedSystemRegistrar<C, R, M, S>,
}

impl<C, R, M, S> GenericSystem for GenericSystemImpl<C, R, M, S>
where
	C: UiSystemContext + 'static,
	R: IntoResult<C::Result, Error: Into<eyre::Report> + core::fmt::Debug> + 'static,
	S: IntoSystem<C, R, M> + Clone + 'static,
{
	fn register(&mut self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			self.registrar.register(world)
		};
		entity_world.insert_if_new(C::Handle::default());
		let mut handle = entity_world
			.get_mut::<C::Handle>()
			.expect("Handle should have just been inserted");
		handle.push(system_id);
	}

	fn unregister(&mut self, entity_world: &mut EntityWorldMut) {
		let system_id = unsafe {
			let world = entity_world.world_mut();
			self.registrar.register(world)
		};
		if let Some(mut handle) = entity_world.get_mut::<C::Handle>()
			&& let Some(i) = handle
				.iter()
				.enumerate()
				.find_map(|(i, &s)| (s == system_id).then_some(i))
		{
			handle.swap_remove(i);
		}
	}
}

enum CachedSystemRegistrar<C, R, M, S>
where
	C: UiSystemContext,
{
	System(S, PhantomData<fn() -> (R, M)>),
	Id(SystemId<C, Result<C::Result, UiSystemError>>),
}

impl<C, R, M, S> CachedSystemRegistrar<C, R, M, S>
where
	C: UiSystemContext + 'static,
	R: IntoResult<C::Result, Error: Into<eyre::Report> + core::fmt::Debug> + 'static,
	S: IntoSystem<C, R, M> + Clone + 'static,
{
	fn register(&mut self, world: &mut World) -> SystemId<C, Result<C::Result, UiSystemError>> {
		match self {
			Self::System(system, _) => {
				let system = system.clone().map(map_system_error);

				let system_id = world.register_system_cached(system);
				let _ = mem::replace(self, Self::Id(system_id));
				system_id
			}
			Self::Id(system_id) => *system_id,
		}
	}
}
