use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::system::Query;
use oprabeli::crossterm::event::{MouseEvent, MouseEventKind};
use oprabeli::ratatui::layout::{Constraint, Flex, Layout, Position};
use oprabeli::ratatui::style::{Color, Stylize as _};
use oprabeli::ratatui::widgets::{Block, Padding, Widget as _, WidgetRef as _};
use oprabeli::{ecs::*, event::SystemEvent};

#[derive(Debug, Clone, Copy)]
pub enum NavbarButtonType {
	Albums,
	Artists,
	Playlists,
}

impl NavbarButtonType {
	pub fn icon(self) -> &'static str {
		match self {
			NavbarButtonType::Albums => "󰀥",
			NavbarButtonType::Artists => "",
			NavbarButtonType::Playlists => "󰲸",
		}
	}

	pub fn text(self) -> &'static str {
		match self {
			NavbarButtonType::Albums => "Albums",
			NavbarButtonType::Artists => "Artists",
			NavbarButtonType::Playlists => "Playlists",
		}
	}
}

#[derive(Debug, Component, Clone, Copy)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::unregister_systems)]
pub struct NavbarButtonComponent {
	button_type: NavbarButtonType,
	hovered: bool,
}

impl UiComponent for NavbarButtonComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::update), UiSystem::new(Self::render)]
	}
}

impl NavbarButtonComponent {
	pub fn new(button_type: NavbarButtonType) -> Self {
		Self {
			button_type,
			hovered: false,
		}
	}

	pub fn button_type(self) -> NavbarButtonType {
		self.button_type
	}

	fn bg_colour(self) -> Option<Color> {
		if self.hovered {
			Some(Color::Black)
		} else {
			None
		}
	}

	fn update(
		context: EventContext<SystemEvent>,
		mut query: Query<(&mut Self, &Area)>,
	) -> eyre::Result<EventFlow> {
		if let SystemEvent::Mouse(MouseEvent {
			kind: MouseEventKind::Moved,
			column: x,
			row: y,
			..
		}) = context.event
		{
			let (mut comp, &Area(area)) = query.get_mut(context.entity)?;
			comp.hovered = area.contains(Position::new(*x, *y));
		}
		Ok(EventFlow::Propagate)
	}

	fn render(context: RenderContext, query: Query<(&Self, &Area)>) -> eyre::Result<()> {
		let (comp, area) = query.get(context.entity)?;
		let area = **area;

		let mut block = Block::new().padding(Padding::horizontal(1));
		if let Some(colour) = comp.bg_colour() {
			block = block.bg(colour);
		}
		block.render_ref(area, context.buffer);

		let [text_area] = Layout::vertical(Constraint::from_lengths([1]))
			.flex(Flex::Center)
			.areas(block.inner(area));
		format!("{} {}", comp.button_type.icon(), comp.button_type.text())
			.bold()
			.render(text_area, context.buffer);

		Ok(())
	}
}
