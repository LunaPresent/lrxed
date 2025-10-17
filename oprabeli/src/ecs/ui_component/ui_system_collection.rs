use core::ops;

use bevy_ecs::component::Component;
use bevy_ecs::system::SystemId;
use smallvec::SmallVec;

use super::*;
use crate::ecs::Area;
use crate::ecs::error_handling::UiSystemError;

const N: usize = 3;

type UiSystemId<C> = SystemId<C, Result<<C as UiSystemContext>::Result, UiSystemError>>;
pub(crate) type InitSystemId = UiSystemId<InitContext>;
pub(crate) type EventSystemId<T> = UiSystemId<EventContext<'static, T>>;
pub(crate) type RenderSystemId = UiSystemId<RenderContext<'static>>;
pub(crate) type ErrorSystemId<E> = UiSystemId<ErrorContext<'static, E>>;

pub(crate) trait UiSystemCollection:
	ops::Deref<Target = SmallVec<[UiSystemId<Self::SystemInput>; N]>>
	+ ops::DerefMut<Target = SmallVec<[UiSystemId<Self::SystemInput>; N]>>
{
	type SystemInput: UiSystemContext + 'static;
	type SystemOutput: 'static;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct InitSystemCollection(SmallVec<[InitSystemId; N]>);

impl UiSystemCollection for InitSystemCollection {
	type SystemInput = InitContext;
	type SystemOutput = <InitContext as UiSystemContext>::Result;
}

#[derive(Debug, Component, Clone, derive_more::Deref, derive_more::DerefMut)]
pub(crate) struct EventSystemCollection<T>(SmallVec<[EventSystemId<T>; N]>)
where
	T: 'static;

impl<T> Default for EventSystemCollection<T> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}

impl<T> UiSystemCollection for EventSystemCollection<T> {
	type SystemInput = EventContext<'static, T>;
	type SystemOutput = <EventContext<'static, T> as UiSystemContext>::Result;
}

#[derive(Debug, Component, Default, Clone, derive_more::Deref, derive_more::DerefMut)]
#[require(Area)]
pub(crate) struct RenderSystemCollection(SmallVec<[RenderSystemId; N]>);

impl UiSystemCollection for RenderSystemCollection {
	type SystemInput = RenderContext<'static>;
	type SystemOutput = <RenderContext<'static> as UiSystemContext>::Result;
}

#[derive(Debug, Component, Clone, derive_more::Deref, derive_more::DerefMut)]
#[require(Area)]
pub(crate) struct ErrorSystemCollection<E>(SmallVec<[ErrorSystemId<E>; N]>)
where
	E: 'static;

impl<E> Default for ErrorSystemCollection<E> {
	fn default() -> Self {
		Self(SmallVec::default())
	}
}
impl<E> UiSystemCollection for ErrorSystemCollection<E> {
	type SystemInput = ErrorContext<'static, E>;
	type SystemOutput = <ErrorContext<'static, E> as UiSystemContext>::Result;
}
