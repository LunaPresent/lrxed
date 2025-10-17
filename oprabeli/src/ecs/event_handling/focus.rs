use bevy_ecs::{entity::Entity, resource::Resource};

/// Global resource that marks the currently focussed entity
///
/// It can be set during app setup, using [`App::with_focussed_component`][wfc]
/// or it can be manually modified in a system using a [`ResMut<Focus>`] parameter
///
/// This resource is automatically added to the world at the start of execution
/// If no entity has been focussed the target will be [`Entity::PLACEHOLDER`]
///
/// [wfc]: crate::tui::app::App::with_focussed_component
#[derive(Debug, Resource)]
pub struct Focus {
	pub target: Entity,
}

impl Default for Focus {
	fn default() -> Self {
		Self {
			target: Entity::PLACEHOLDER,
		}
	}
}
