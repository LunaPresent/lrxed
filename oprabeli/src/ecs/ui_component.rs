mod context;
mod ui_system;
mod ui_system_collection;

pub use context::*;
pub use ui_system::*;
pub(crate) use ui_system_collection::*;

use bevy_ecs::lifecycle::HookContext;
use bevy_ecs::world::{DeferredWorld, EntityWorldMut, World};

// TODO: documentation
pub trait UiComponent {
	// TODO: documentation
	fn systems() -> impl IntoIterator<Item = UiSystem>;

	// TODO: documentation
	fn register_systems(mut world: DeferredWorld, context: HookContext) {
		let mut cmd = world.commands();
		let mut entity_cmd = cmd.entity(context.entity);
		entity_cmd.queue(move |mut entity_world: EntityWorldMut| {
			for mut system in Self::systems() {
				system.register(&mut entity_world);
			}
		});
	}

	// TODO: documentation
	fn unregister_systems(mut world: DeferredWorld, context: HookContext) {
		let mut cmd = world.commands();
		cmd.queue(move |world: &mut World| {
			if let Ok(mut entity_world) = world.get_entity_mut(context.entity) {
				for mut system in Self::systems() {
					system.unregister(&mut entity_world);
				}
			}
		});
	}
}
