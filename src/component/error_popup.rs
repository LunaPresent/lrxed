use std::time::Duration;

use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::hierarchy::ChildOf;
use oprabeli::bevy_ecs::system::{Commands, Query, Res};
use oprabeli::ratatui::style::Stylize as _;
use oprabeli::ratatui::widgets::{Block, BorderType, Clear, Paragraph, Widget as _};
use oprabeli::{ecs::*, event::SystemEvent};

use super::ErrorReporterComponent;
use crate::config::Theme;

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
#[require(ZOrder(100))]
pub struct ErrorPopupComponent {
	error_msg: String,
	ttl: Duration,
}

impl UiComponent for ErrorPopupComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::update), UiSystem::new(Self::render)]
	}
}

impl ErrorPopupComponent {
	pub fn new(error_msg: String, ttl: Duration) -> Self {
		Self { error_msg, ttl }
	}

	fn update(
		context: EventContext<SystemEvent>,
		mut query: Query<(&mut Self, &ChildOf)>,
		mut loggers: Query<&mut ErrorReporterComponent>,
		mut cmd: Commands,
	) -> eyre::Result<EventFlow> {
		if let &SystemEvent::Tick(duration) = context.event {
			let (mut comp, parent) = query.get_mut(context.entity)?;
			comp.ttl = comp.ttl.saturating_sub(duration);

			if comp.ttl == Duration::ZERO {
				let mut logger = loggers.get_mut(parent.parent())?;
				logger.error_popups.retain(|&e| e != context.entity);
				let mut ec = cmd.entity(context.entity);
				ec.despawn();
			}
		}
		Ok(EventFlow::Propagate)
	}

	fn render(
		context: RenderContext,
		theme: Res<Theme>,
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (comp, area) = query.get(context.entity)?;
		let area = **area;
		Clear.render(area, context.buffer);
		Paragraph::new(comp.error_msg.as_str())
			.block(
				Block::bordered()
					.border_style(theme.colours.border_error)
					.border_type(BorderType::Rounded),
			)
			.bg(theme.colours.background)
			.render(area, context.buffer);

		Ok(())
	}
}
