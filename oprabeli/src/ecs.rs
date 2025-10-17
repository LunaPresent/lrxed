mod bevy_error_wrapper;
mod entity_commands_ext;
mod error_handling;
mod event_handling;
mod init;
mod into_result;
mod rendering;
mod signal;
mod ui_component;

pub use bevy_error_wrapper::BevyErrorWrapper;
pub use entity_commands_ext::EntityCommandsExt;
pub use error_handling::ErrorFlow;
pub(crate) use event_handling::DynEventDispatch;
pub use event_handling::{AsyncEventQueue, EventFlow, EventQueue, Focus};
pub use rendering::{Area, Viewport, ZOrder};
pub use signal::Signal;
pub use ui_component::{
	ErrorContext, EventContext, InitContext, RenderContext, UiComponent, UiSystem,
};

use bevy_ecs::bundle::Bundle;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::World;
use ratatui::Frame;
use tokio::sync::mpsc;

use super::app::AppControls;
use super::event::*;
use event_handling::EventDispatcher;
use init::init_components;
use rendering::Renderer;

#[derive(Debug)]
pub(crate) struct ComponentSystem {
	world: World,
	event_dispatcher: EventDispatcher,
	renderer: Renderer,
}

impl ComponentSystem {
	pub fn new(
		controls: AppControls,
		event_sender: mpsc::UnboundedSender<DynEventDispatch>,
	) -> Self {
		let mut world = World::new();
		world.insert_resource(Signal::new(controls));
		world.insert_resource(Focus::default());
		world.insert_resource(EventQueue::default());
		world.insert_resource(AsyncEventQueue::new(event_sender));

		ComponentSystem {
			world,
			event_dispatcher: EventDispatcher::default(),
			renderer: Default::default(),
		}
	}

	pub fn add_entity(&mut self) -> Entity {
		self.world.spawn_empty().id()
	}

	pub fn add_component(&mut self, entity: Entity, component_bundle: impl Bundle) {
		self.world.entity_mut(entity).insert(component_bundle);
	}

	pub fn init(&mut self) -> eyre::Result<()> {
		self.world.flush();
		self.world.run_system_cached(init_components)?;
		Ok(())
	}

	pub fn dispatch_system_event(
		&mut self,
		ed: EventDispatch<SystemEvent>,
	) -> eyre::Result<Option<SystemEvent>> {
		let event = self.event_dispatcher.dispatch(ed, &mut self.world)?;
		self.world.run_system_cached(init_components)?;

		while let Some(ed) = self
			.world
			.get_resource_mut::<EventQueue>()
			.and_then(|mut queue| queue.next())
		{
			self.event_dispatcher.dispatch_dyn(ed, &mut self.world)?;
		}

		Ok(event)
	}

	pub fn dispatch_dyn_event(&mut self, ed: DynEventDispatch) -> eyre::Result<()> {
		self.event_dispatcher.dispatch_dyn(ed, &mut self.world)?;
		self.world.run_system_cached(init_components)?;

		while let Some(ed) = self
			.world
			.get_resource_mut::<EventQueue>()
			.and_then(|mut queue| queue.next())
		{
			self.event_dispatcher.dispatch_dyn(ed, &mut self.world)?;
		}

		Ok(())
	}

	pub fn draw(&mut self, frame: &mut Frame) -> eyre::Result<()> {
		let area = frame.area();
		self.renderer
			.render(frame.buffer_mut(), area, &mut self.world)
	}
}
