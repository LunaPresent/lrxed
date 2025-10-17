use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::entity::Entity;
use oprabeli::bevy_ecs::system::{Commands, Query};
use oprabeli::ecs::*;
use oprabeli::ratatui::layout::{Constraint, Layout};

use super::{NavbarButtonComponent, navbar_button::NavbarButtonType};

#[derive(Debug, Component, Default)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::register_systems)]
pub struct NavbarComponent {
	buttons: Vec<Entity>,
}

impl UiComponent for NavbarComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::init), UiSystem::new(Self::render)]
	}
}

impl NavbarComponent {
	fn init(
		context: InitContext,
		mut query: Query<&mut Self>,
		mut cmd: Commands,
	) -> eyre::Result<()> {
		let mut comp = query.get_mut(context.entity)?;
		let mut ec = cmd.entity(context.entity);

		comp.buttons.reserve(3);
		comp.buttons.push(
			ec.spawn_child(NavbarButtonComponent::new(NavbarButtonType::Albums))
				.id(),
		);
		comp.buttons.push(
			ec.spawn_child(NavbarButtonComponent::new(NavbarButtonType::Artists))
				.id(),
		);
		comp.buttons.push(
			ec.spawn_child(NavbarButtonComponent::new(NavbarButtonType::Playlists))
				.id(),
		);

		Ok(())
	}

	fn render(
		context: RenderContext,
		query: Query<&Self>,
		mut areas: Query<&mut Area>,
		buttons: Query<&NavbarButtonComponent>,
	) -> eyre::Result<()> {
		let comp = query.get(context.entity)?;
		let area = **areas.get(context.entity)?;

		let button_areas = Layout::horizontal(Constraint::from_lengths(comp.buttons.iter().map(
			|entity| {
				buttons
					.get(*entity)
					.map(|btn| btn.button_type().text().len() as u16 + 4)
					.unwrap_or_default()
			},
		)))
		.spacing(1)
		.split(area);
		for (&button, &button_area) in comp.buttons.iter().zip(button_areas.iter()) {
			**areas.get_mut(button)? = button_area;
		}

		Ok(())
	}
}
