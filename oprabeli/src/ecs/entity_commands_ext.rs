use bevy_ecs::bundle::Bundle;
use bevy_ecs::hierarchy::ChildOf;
use bevy_ecs::relationship::Relationship;
use bevy_ecs::system::EntityCommands;

// TODO: documentation
pub trait EntityCommandsExt {
	fn spawn_child(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
	fn spawn_related<R: Relationship>(&mut self, bundle: impl Bundle) -> EntityCommands<'_>;
}

impl EntityCommandsExt for EntityCommands<'_> {
	fn spawn_child(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
		self.spawn_related::<ChildOf>(bundle)
	}

	fn spawn_related<R: Relationship>(&mut self, bundle: impl Bundle) -> EntityCommands<'_> {
		let parent = self.id();
		self.commands_mut().spawn((bundle, R::from(parent)))
	}
}
