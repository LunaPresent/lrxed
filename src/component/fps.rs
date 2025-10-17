use std::{
	collections::VecDeque,
	time::{Duration, Instant},
};

use color_eyre::eyre;
use oprabeli::bevy_ecs;
use oprabeli::bevy_ecs::component::Component;
use oprabeli::bevy_ecs::system::Query;
use oprabeli::ecs::*;
use oprabeli::ratatui::layout::{Constraint, Flex, Layout};
use oprabeli::ratatui::text::Span;
use oprabeli::ratatui::widgets::Widget;

use crate::util::{Direction as _, OctDirection};

#[derive(Debug, Component)]
#[component(on_add = Self::register_systems)]
#[component(on_remove = Self::register_systems)]
#[require(ZOrder(9999))]
pub struct FpsComponent {
	position: OctDirection,
	time_history: VecDeque<Instant>,
}

impl UiComponent for FpsComponent {
	fn systems() -> impl IntoIterator<Item = UiSystem> {
		[UiSystem::new(Self::render)]
	}
}

impl FpsComponent {
	pub fn new(position: OctDirection) -> Self {
		let mut time_history = VecDeque::with_capacity(60);
		time_history.push_back(Instant::now());
		FpsComponent {
			position,
			time_history,
		}
	}

	fn record_frame(&mut self) -> Duration {
		let now = Instant::now();
		let interval = now.duration_since(self.time_history[0]);
		if self.time_history.len() == self.time_history.capacity() {
			self.time_history.pop_front();
		}
		self.time_history.push_back(now);
		interval
	}

	fn render(context: RenderContext, mut query: Query<(&mut Self, &Area)>) -> eyre::Result<()> {
		let (mut comp, &Area(area)) = query.get_mut(context.entity)?;

		let interval = comp.record_frame().as_secs_f64();
		let fps = Span::from(format!(
			"{} fps",
			(comp.time_history.len() as f64 / interval).round()
		));

		let flex_horizontal = Self::i16_to_flex(comp.position.x());
		let flex_vertical = Self::i16_to_flex(comp.position.y());
		let [area] = Layout::horizontal([Constraint::Length(fps.width() as u16)])
			.flex(flex_horizontal)
			.areas(area);
		let [area] = Layout::vertical([Constraint::Length(1)])
			.flex(flex_vertical)
			.areas(area);

		fps.render(area, context.buffer);

		Ok(())
	}

	fn i16_to_flex(n: i16) -> Flex {
		match n {
			..0 => Flex::Start,
			0 => Flex::Center,
			1.. => Flex::End,
		}
	}
}
