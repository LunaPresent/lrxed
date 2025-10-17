use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;

use super::App;

#[derive(Debug)]
pub struct EntityBuilder {
	entity: Entity,
	app: App,
}

impl EntityBuilder {
	pub fn new(entity: Entity, app: App) -> Self {
		Self { entity, app }
	}

	pub fn id(&self) -> Entity {
		self.entity
	}

	pub fn app(self) -> App {
		self.app
	}

	pub fn with_child(mut self, f: impl FnOnce(Self) -> eyre::Result<Self>) -> eyre::Result<Self> {
		let entity = self.entity;
		let app = (f)(EntityBuilder::new(self.app.ecs.add_entity(), self.app))?.app;
		Ok(Self::new(entity, app))
	}

	pub fn with_component(mut self, component_bundle: impl Component) -> eyre::Result<Self> {
		self.app.ecs.add_component(self.entity, component_bundle);
		self.app.ecs.init()?;
		Ok(self)
	}
}
