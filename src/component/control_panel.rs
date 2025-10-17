use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::system::{Query, Res};
use oprabeli::crossterm::event::{MouseButton, MouseEventKind};
use oprabeli::ecs::*;
use oprabeli::event::SystemEvent;
use oprabeli::ratatui::layout::{Constraint, Flex, Layout};
use oprabeli::ratatui::style::Stylize as _;
use oprabeli::ratatui::widgets::{Block, Widget};

use crate::config::Theme;

#[derive(Debug, Component, Default, Clone, Copy)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct ControlPanelComponent {
	playing: bool,
}

impl UiComponent for ControlPanelComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::update), UiSystem::new(Self::render)]
	}
}

impl ControlPanelComponent {
	fn icon(self) -> &'static str {
		if self.playing { "󰏤" } else { "󰐊" }
	}

	fn update(
		context: EventContext<SystemEvent>,
		mut query: Query<&mut Self>,
	) -> eyre::Result<EventFlow> {
		let mut comp = query.get_mut(context.entity)?;

		Ok(match context.event {
			SystemEvent::Mouse(mouse_event) => match mouse_event.kind {
				MouseEventKind::Down(MouseButton::Left) => {
					comp.playing = !comp.playing;
					EventFlow::Consume
				}
				_ => EventFlow::Propagate,
			},
			_ => EventFlow::Propagate,
		})
	}

	fn render(
		context: RenderContext,
		theme: Res<Theme>,
		query: Query<(&Self, &Area)>,
	) -> eyre::Result<()> {
		let (comp, area) = query.get(context.entity)?;
		let area = **area;

		Block::new()
			.bg(theme.colours.overlay)
			.render(area, context.buffer);

		let [button_area] = Layout::vertical([Constraint::Length(1)])
			.flex(Flex::Center)
			.areas(area);
		let [button_area] = Layout::horizontal([Constraint::Length(2)])
			.flex(Flex::Center)
			.areas(button_area);

		comp.icon().render(button_area, context.buffer);

		Ok(())
	}
}
