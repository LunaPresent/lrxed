use bevy_ecs::entity::Entity;
use bevy_ecs::hierarchy::{ChildOf, Children};
use bevy_ecs::query::Without;
use bevy_ecs::system::{In, InMut, Local, Query, Res, RunSystemOnce as _};
use bevy_ecs::world::World;
use ratatui::layout::Position;

use super::{EventFlow, Focus};
use crate::ecs::error_handling::UiSystemResultExt as _;
use crate::ecs::ui_component::*;
use crate::ecs::*;
use crate::event::{CursorAction, EventDispatch};

#[derive(Debug)]
struct EntityEventInfo {
	entity: Entity,
	system_id: Entity,
}

#[derive(Debug, Default)]
pub(crate) struct EventDispatcher {
	entity_system_queue: Vec<EntityEventInfo>,
}

impl EventDispatcher {
	pub fn dispatch<T>(
		&mut self,
		ed: EventDispatch<T>,
		world: &mut World,
	) -> eyre::Result<Option<T>>
	where
		T: Send + Sync + 'static,
	{
		self.entity_system_queue.clear();
		ed.dispatch(&mut self.entity_system_queue, world)
	}

	pub fn dispatch_dyn(
		&mut self,
		mut ed: DynEventDispatch,
		world: &mut World,
	) -> eyre::Result<()> {
		self.entity_system_queue.clear();
		ed.inner.dispatch_dyn(&mut self.entity_system_queue, world)
	}
}

pub(crate) struct DynEventDispatch {
	inner: Box<dyn RunEventSystems>,
}

impl DynEventDispatch {
	pub fn new<T>(dispatch: DispatchMethod, event: T) -> Self
	where
		T: Send + Sync + 'static,
	{
		Self {
			inner: Box::new(Some(EventDispatch::new(dispatch, event))),
		}
	}
}

trait RunEventSystems: Send + Sync {
	fn dispatch_dyn(
		&mut self,
		queue: &mut Vec<EntityEventInfo>,
		world: &mut World,
	) -> eyre::Result<()>;
}

impl<T> RunEventSystems for Option<EventDispatch<T>>
where
	T: Send + Sync + 'static,
{
	fn dispatch_dyn(
		&mut self,
		queue: &mut Vec<EntityEventInfo>,
		world: &mut World,
	) -> eyre::Result<()> {
		let this = self
			.take()
			.expect("run_event_systems should only be called once");

		this.dispatch(queue, world).map(|_| ())
	}
}

impl<T> EventDispatch<T>
where
	T: Send + Sync + 'static,
{
	fn dispatch(
		self,
		queue: &mut Vec<EntityEventInfo>,
		world: &mut World,
	) -> eyre::Result<Option<T>> {
		match self.dispatch {
			DispatchMethod::Input => world
				.run_system_once_with(Self::find_input_entities, queue)
				.map_err(BevyErrorWrapper::from)?,
			DispatchMethod::Broadcast => world
				.run_system_once_with(Self::find_broadcast_entities, queue)
				.map_err(BevyErrorWrapper::from)?,
			DispatchMethod::Cursor { x, y, action } => world
				.run_system_cached_with(Self::find_cursor_entities, (queue, x, y, action))??,
			DispatchMethod::Target(target) => world
				.run_system_once_with(Self::find_target_entities, (queue, target))
				.map_err(BevyErrorWrapper::from)?,
		}

		Ok(match self.dispatch {
			DispatchMethod::Broadcast => {
				for target in queue {
					world
						.run_system_with(
							EventSystemId::<T>::from_entity(target.system_id),
							EventContext {
								entity: target.entity,
								event: &self.event,
							},
						)?
						.handle(target.entity, world)?;
				}
				Some(self.event)
			}
			_ => Self::target_dispatch(queue.as_slice(), self.event, world)?,
		})
	}

	fn target_dispatch(
		queue: &[EntityEventInfo],
		event: T,
		world: &mut World,
	) -> eyre::Result<Option<T>> {
		let mut consume = false;
		let mut prev_entity = Entity::PLACEHOLDER;

		for target in queue {
			// make multiple systems on one entity order independent by firing all of them even if
			// one returned consume
			if consume && prev_entity != target.entity {
				break;
			}
			prev_entity = target.entity;

			let flow = world
				.run_system_with(
					EventSystemId::<T>::from_entity(target.system_id),
					EventContext {
						entity: target.entity,
						event: &event,
					},
				)?
				.handle(target.entity, world)?;
			match flow {
				Some(EventFlow::Propagate) => (),
				_ => consume = true,
			}
		}
		if consume { Ok(None) } else { Ok(Some(event)) }
	}

	fn find_input_entities(
		InMut(targets): InMut<Vec<EntityEventInfo>>,
		focus: Res<Focus>,
		systems: Query<&EventSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(focus.target, targets, systems, parents);
	}

	fn find_broadcast_entities(
		InMut(targets): InMut<Vec<EntityEventInfo>>,
		components: Query<(Entity, &EventSystemCollection<T>)>,
	) {
		for (entity, systems) in components {
			for &system in systems.iter() {
				targets.push(EntityEventInfo {
					entity,
					system_id: system.entity(),
				});
			}
		}
	}

	#[allow(
		clippy::type_complexity,
		reason = "separating the tuple into a typedef makes it less clear what is going on"
	)]
	#[allow(
		clippy::too_many_arguments,
		reason = "most of the arguments are injected by bevy"
	)]
	fn find_cursor_entities(
		(InMut(targets), In(x), In(y), In(action)): (
			InMut<Vec<EntityEventInfo>>,
			In<u16>,
			In<u16>,
			In<CursorAction>,
		),
		mut clicked: Local<Option<Entity>>,
		root_entities: Query<Entity, Without<ChildOf>>,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
		systems: Query<&EventSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) -> eyre::Result<()> {
		let target = if clicked.is_some() {
			*clicked
		} else {
			Self::find_cursor_target(x, y, root_entities, areas)?
		};
		match action {
			CursorAction::Engage => *clicked = target,
			CursorAction::Release => *clicked = None,
			_ => (),
		}

		if let Some(target) = target {
			Self::bubble_entities(target, targets, systems, parents);
		}
		Ok(())
	}

	fn find_target_entities(
		(InMut(targets), In(target)): (InMut<Vec<EntityEventInfo>>, In<Entity>),
		handles: Query<&EventSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) {
		Self::bubble_entities(target, targets, handles, parents);
	}

	fn bubble_entities(
		head: Entity,
		targets: &mut Vec<EntityEventInfo>,
		systems: Query<&EventSystemCollection<T>>,
		parents: Query<&ChildOf>,
	) {
		if let Ok(systems) = systems.get(head) {
			for &system in systems.iter() {
				targets.push(EntityEventInfo {
					entity: head,
					system_id: system.entity(),
				});
			}
		}
		if let Ok(parent) = parents.get(head) {
			Self::bubble_entities(parent.parent(), targets, systems, parents);
		}
	}

	fn find_cursor_target(
		x: u16,
		y: u16,
		root_entities: Query<Entity, Without<ChildOf>>,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
	) -> eyre::Result<Option<Entity>> {
		for entity in root_entities {
			if let Some(target) = Self::find_cursor_target_inner(Position { x, y }, entity, areas)?
			{
				return Ok(Some(target));
			}
		}
		Ok(None)
	}

	/// result is based on the following "truth table"
	///
	/// | has area | in area | has children | result........... |
	/// | -------- | ------- | ------------ | ----------------- |
	/// | .......0 | ....../ | ...........0 | none............. |
	/// | .......1 | ......0 | ...........0 | none............. |
	/// | .......1 | ......1 | ...........0 | self............. |
	/// | .......0 | ....../ | ...........1 | recurse then none |
	/// | .......1 | ......0 | ...........1 | none............. |
	/// | .......1 | ......1 | ...........1 | recurse then self |
	fn find_cursor_target_inner(
		mut pos: Position,
		entity: Entity,
		areas: Query<(Option<&Area>, Option<&Children>, Option<&Viewport>)>,
	) -> eyre::Result<Option<Entity>> {
		let (area, children, viewport) = areas.get(entity)?;
		if let Some(area) = area
			&& !area.contains(pos)
		{
			Ok(None)
		} else {
			if let (Some(area), Some(viewport)) = (area, viewport) {
				pos.x = pos.x - area.x + viewport.offset.x;
				pos.y = pos.y - area.y + viewport.offset.y;
			}
			if let Some(children) = children {
				for &child in children {
					if let Some(target) = Self::find_cursor_target_inner(pos, child, areas)? {
						return Ok(Some(target));
					}
				}
			}
			if area.is_some() {
				Ok(Some(entity))
			} else {
				Ok(None)
			}
		}
	}
}
