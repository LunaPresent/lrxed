use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::entity::Entity;
use oprabeli::bevy_ecs::system::Query;
use oprabeli::ecs::*;
use oprabeli::ratatui::layout::{Rect, Size};

use crate::app_event::AppEvent;
use crate::util::{Direction as _, IntoOffset as _, ResetOrigin as _};

#[derive(Debug, Component)]
#[require(Viewport)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ScrollableComponent<F>
where
	F: Fn(Rect) -> Size + Send + Sync + 'static,
{
	inner: Entity,
	size_fn: F,
}

impl<F> UiComponent for ScrollableComponent<F>
where
	F: Fn(Rect) -> Size + Send + Sync + 'static,
{
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::update), UiSystem::new(Self::render)]
	}
}

impl<F> ScrollableComponent<F>
where
	F: Fn(Rect) -> Size + Send + Sync + 'static,
{
	pub fn new(inner: Entity, size_fn: F) -> Self {
		Self { inner, size_fn }
	}

	fn update(
		context: EventContext<AppEvent>,
		mut query: Query<(&mut Viewport, &Area)>,
	) -> eyre::Result<EventFlow> {
		let (mut viewport, area) = query.get_mut(context.entity)?;
		Ok(match context.event {
			AppEvent::ScrollBy { direction, amount } => {
				Self::scroll(
					viewport.as_mut(),
					direction.x() * amount.cast_signed(),
					direction.y() * amount.cast_signed(),
				);
				viewport.clamp_offset(area.as_size())?;
				EventFlow::Consume
			}
			AppEvent::ScrollByRelative {
				direction,
				fraction,
			} => {
				let size = area.as_size();
				Self::scroll(
					viewport.as_mut(),
					direction.x() * (size.width as f32 * fraction) as i16,
					direction.y() * (size.height as f32 * fraction) as i16,
				);
				viewport.clamp_offset(size)?;
				EventFlow::Consume
			}
			AppEvent::ScrollTo(rect) => {
				let area = area.reset_origin().offset(viewport.offset.into_offset());
				viewport.offset.x -= area.left().saturating_sub(rect.left());
				viewport.offset.x += rect.right().saturating_sub(area.right());
				viewport.offset.y -= area.top().saturating_sub(rect.top());
				viewport.offset.y += rect.bottom().saturating_sub(area.bottom());
				viewport.clamp_offset(area.as_size())?;
				EventFlow::Consume
			}
			_ => EventFlow::Propagate,
		})
	}

	fn render(
		context: RenderContext,
		mut query: Query<(&Self, &mut Viewport)>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let (comp, mut viewport) = query.get_mut(context.entity)?;
		let area = **areas.get(context.entity)?;
		viewport.size = (comp.size_fn)(area);
		viewport.clamp_offset(area.as_size())?;
		**areas.get_mut(comp.inner)? = viewport.area();

		Ok(())
	}

	fn scroll(viewport: &mut Viewport, x: i16, y: i16) {
		viewport.offset.x = viewport.offset.x.saturating_add_signed(x);
		viewport.offset.y = viewport.offset.y.saturating_add_signed(y);
	}
}
