use bevy_ecs::component::{Component, Mutable};
use bevy_ecs::entity::Entity;
use bevy_ecs::system::SystemInput;
use ratatui::buffer::Buffer;

use super::*;
use crate::ecs::EventFlow;
use crate::ecs::error_handling::ErrorFlow;

pub(crate) trait UiSystemContext: SystemInput {
	type Result;
	type Handle: UiSystemCollection<SystemInput = Self, SystemOutput = Self::Result>
		+ Component<Mutability = Mutable>
		+ Default;
}

#[derive(Debug, Clone, Copy)]
pub struct InitContext {
	pub entity: Entity,
}

impl UiSystemContext for InitContext {
	type Result = ();
	type Handle = InitSystemCollection;
}

impl SystemInput for InitContext {
	type Param<'i> = InitContext;
	type Inner<'i> = InitContext;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}
#[derive(Debug)]
pub struct EventContext<'a, T> {
	pub entity: Entity,
	pub event: &'a T,
}

impl<T> UiSystemContext for EventContext<'static, T>
where
	T: 'static,
{
	type Result = EventFlow;
	type Handle = EventSystemCollection<T>;
}

impl<T> SystemInput for EventContext<'_, T>
where
	T: 'static,
{
	type Param<'i> = EventContext<'i, T>;
	type Inner<'i> = EventContext<'i, T>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}

#[derive(Debug)]
pub struct RenderContext<'a> {
	pub entity: Entity,
	pub buffer: &'a mut Buffer,
}

impl UiSystemContext for RenderContext<'static> {
	type Result = ();
	type Handle = RenderSystemCollection;
}

impl SystemInput for RenderContext<'_> {
	type Param<'i> = RenderContext<'i>;
	type Inner<'i> = RenderContext<'i>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}

#[derive(Debug)]
pub struct ErrorContext<'a, E>
where
	E: 'static,
{
	pub entity: Entity,
	pub error: &'a E,
}

impl<E> UiSystemContext for ErrorContext<'static, E>
where
	E: 'static,
{
	type Result = ErrorFlow;
	type Handle = ErrorSystemCollection<E>;
}

impl<E> SystemInput for ErrorContext<'_, E>
where
	E: 'static,
{
	type Param<'i> = ErrorContext<'i, E>;
	type Inner<'i> = ErrorContext<'i, E>;

	fn wrap(this: Self::Inner<'_>) -> Self::Param<'_> {
		this
	}
}
