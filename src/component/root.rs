use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::entity::Entity;
use oprabeli::bevy_ecs::system::{Commands, Query, Res, ResMut};
use oprabeli::config::KeyHandler;
use oprabeli::ecs::*;
use oprabeli::ratatui::layout::{Constraint, Layout, Size};
use oprabeli::ratatui::style::Stylize;
use oprabeli::ratatui::widgets::{Block, Widget};

use super::{
	ControlPanelComponent, ErrorReporterComponent, LibraryComponent, NavbarComponent,
	ScrollableComponent,
};
use crate::config::{Keys, Theme};
use crate::event::AppEvent;

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct RootComponent {
	control_panel: Entity,
	nav_bar: Entity,
	library_scrollable: Entity,
}

impl UiComponent for RootComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[
			UiSystem::new(Self::init),
			UiSystem::new(Self::update),
			UiSystem::new(Self::render),
		]
	}
}

impl Default for RootComponent {
	fn default() -> Self {
		Self {
			control_panel: Entity::PLACEHOLDER,
			nav_bar: Entity::PLACEHOLDER,
			library_scrollable: Entity::PLACEHOLDER,
		}
	}
}

impl RootComponent {
	fn init(
		context: InitContext,
		key_config: Res<Keys>,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(context.entity)?;
		let library = cmd.spawn(LibraryComponent::default()).id();

		let mut ec = cmd.entity(context.entity);
		ec.insert_if_new(ErrorReporterComponent::new());
		ec.insert_if_new(KeyHandler::new(key_config.global.try_generate_key_map()?));
		comp.control_panel = ec.spawn_child(ControlPanelComponent::default()).id();
		comp.nav_bar = ec.spawn_child(NavbarComponent::default()).id();
		let mut scrollable = ec.spawn_child(ScrollableComponent::new(library, |rect| {
			Size::new(rect.width, rect.height * 3)
		}));
		comp.library_scrollable = scrollable.id();
		scrollable.add_child(library);

		Ok(())
	}

	fn update(
		context: EventContext<AppEvent>,
		key_config: Res<Keys>,
		mut signal: ResMut<Signal>,
		mut key_handler_query: Query<&mut KeyHandler<AppEvent>>,
	) -> eyre::Result<EventFlow> {
		Ok(match context.event {
			AppEvent::Quit => {
				signal.quit()?;
				EventFlow::Consume
			}
			AppEvent::Suspend => {
				signal.suspend()?;
				EventFlow::Consume
			}
			AppEvent::UpdateKeymap => {
				let mut key_handler = key_handler_query.get_mut(context.entity)?;
				*key_handler = KeyHandler::new(key_config.global.try_generate_key_map()?);
				EventFlow::Propagate
			}
			_ => EventFlow::Propagate,
		})
	}

	fn render(
		context: RenderContext,
		theme: Res<Theme>,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
	) -> eyre::Result<()> {
		let comp = query.get(context.entity)?;
		let area = **areas.get(context.entity)?;

		Block::new()
			.bg(theme.colours.background)
			.render(area, context.buffer);

		let [navbar_area, library_area, control_panel_area] = Layout::vertical([
			Constraint::Length(1),
			Constraint::Fill(1),
			Constraint::Length(5),
		])
		.areas(area);

		**areas.get_mut(comp.nav_bar)? = navbar_area;
		**areas.get_mut(comp.control_panel)? = control_panel_area;
		**areas.get_mut(comp.library_scrollable)? = library_area;

		Ok(())
	}
}
