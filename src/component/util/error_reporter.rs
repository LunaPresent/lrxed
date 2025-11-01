use std::collections::VecDeque;

use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::entity::Entity;
use oprabeli::bevy_ecs::system::{Commands, Query, Res};
use oprabeli::ecs::*;
use oprabeli::ratatui::layout::{Constraint, Flex, Layout};

use super::ErrorPopupComponent;
use crate::config::Settings;

#[derive(Debug, Component, Default)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ErrorReporterComponent {
	pub error_popups: VecDeque<Entity>,
}

impl UiComponent for ErrorReporterComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[
			UiSystem::new(Self::render),
			UiSystem::new(Self::handle_error),
		]
	}
}

impl ErrorReporterComponent {
	pub fn new() -> Self {
		Self::default()
	}

	fn render(
		context: RenderContext,
		query: Query<&Self>,
		mut widgets: Query<&mut EcsWidget>,
	) -> eyre::Result<()> {
		let comp = query.get(context.entity)?;
		let [error_area] = Layout::horizontal([Constraint::Max(60)])
			.flex(Flex::End)
			.areas(context.area);
		let error_areas = Layout::vertical(core::iter::repeat_n(
			Constraint::Max(7),
			comp.error_popups.len(),
		))
		.split(error_area);
		for (&error_popup, &area) in comp.error_popups.iter().zip(error_areas.iter()) {
			widgets.get_mut(error_popup)?.render(area);
		}
		Ok(())
	}

	fn handle_error(
		context: ErrorContext<eyre::Report>,
		settings: Res<Settings>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<ErrorFlow> {
		let mut comp = query.get_mut(context.entity)?;
		let mut ec = cmd.entity(context.entity);
		comp.error_popups.push_back(
			ec.spawn_child(ErrorPopupComponent::new(
				context.error.to_string(),
				settings.notification_timeout,
			))
			.id(),
		);

		let _ = ErrorFlow::Explode;
		let _ = ErrorFlow::Propagate;
		Ok(ErrorFlow::Catch)
	}
}
