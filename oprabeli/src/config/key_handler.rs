use std::time::Duration;

use bevy_ecs::component::Component;
use bevy_ecs::system::{Query, ResMut};

use super::{KeyChord, KeyMap, KeyMapMatch};
use crate::ecs::*;
use crate::event::*;

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct KeyHandler<T>
where
	T: Send + Sync + Clone + 'static,
{
	key_map: KeyMap<T>,
	key_map_match: KeyMapMatch,
	timeout: Duration,
	timeoutlen: Duration,
}

impl<T> UiComponent for KeyHandler<T>
where
	T: Send + Sync + Clone + 'static,
{
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::update)]
	}
}

impl<T> KeyHandler<T>
where
	T: Send + Sync + Clone + 'static,
{
	pub fn new(key_map: KeyMap<T>) -> Self {
		Self {
			key_map,
			key_map_match: KeyMapMatch::new(),
			timeout: Duration::ZERO,
			timeoutlen: Duration::from_secs(1),
		}
	}

	fn update(
		context: EventContext<SystemEvent>,
		mut event_queue: ResMut<EventQueue>,
		mut query: Query<&mut Self>,
	) -> EventFlow {
		let mut comp = query
			.get_mut(context.entity)
			.expect("Self type component should be present on the entity");

		match context.event {
			SystemEvent::Tick(delta) => {
				if !comp.key_map_match.matches(&comp.key_map).is_empty() {
					comp.timeout += *delta;
				}
				if comp.timeout > comp.timeoutlen {
					let matches = comp.key_map_match.full_matches(&comp.key_map);
					for app_event in matches.iter().map(|m| m.app_event.clone()) {
						event_queue.send(DispatchMethod::Input, app_event);
					}
					comp.key_map_match = KeyMapMatch::new();
					comp.timeout = Duration::ZERO;
				}
				EventFlow::Propagate
			}
			SystemEvent::Key(key_event) => {
				comp.timeout = Duration::ZERO;
				let key_chord = KeyChord::from_event(*key_event);
				comp.key_map_match = comp.key_map.match_key(key_chord, comp.key_map_match);

				if comp.key_map_match.matches(&comp.key_map).is_empty() {
					comp.key_map_match = KeyMapMatch::new();
					EventFlow::Propagate
				} else if comp.key_map_match.partial_matches(&comp.key_map).is_empty() {
					let matches = comp.key_map_match.full_matches(&comp.key_map);
					for app_event in matches.iter().map(|m| m.app_event.clone()) {
						event_queue.send(DispatchMethod::Input, app_event);
					}
					comp.key_map_match = KeyMapMatch::new();
					EventFlow::Consume
				} else {
					EventFlow::Consume
				}
			}
			_ => EventFlow::Propagate,
		}
	}
}
