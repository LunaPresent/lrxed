use core::{fmt, str};

use serde_with::chrono::TimeDelta;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::parameter::CommandParameterKind;
use super::{CommandIntoAppEventError, CommandKind, CommandParameter, ParseCommandError};
use crate::command::parameter::{
	IntegerParameter, ParameterPolarity, PercentageParameter, PlaybackParameter, TimeParameter,
};
use crate::event::AppEvent;
use crate::util::BoolModifier;

#[derive(Debug, Clone, Copy, PartialEq, SerializeDisplay, DeserializeFromStr)]
pub struct Command {
	pub kind: CommandKind,
	pub parameter: Option<CommandParameter>,
}

impl Command {
	pub fn try_into_app_event(self) -> Result<AppEvent, CommandIntoAppEventError> {
		match self.kind {
			CommandKind::Save => self.convert_save(),
			CommandKind::Playback => self.convert_playback(),
			CommandKind::Seek => self.convert_seek(),
			CommandKind::Volume => self.convert_volume(),
			CommandKind::Speed => self.convert_speed(),
			CommandKind::Timestamp => self.convert_timestamp(),
		}
		.ok_or(CommandIntoAppEventError::BadCommand(self))
	}

	fn convert_save(self) -> Option<AppEvent> {
		self.parameter.is_none().then_some(AppEvent::Save)
	}

	fn convert_playback(self) -> Option<AppEvent> {
		Some(match self.parameter? {
			CommandParameter::Playback(p) => AppEvent::SetPlayback(match p {
				PlaybackParameter::Toggle => BoolModifier::Toggle,
				PlaybackParameter::Play => BoolModifier::SetTrue,
				PlaybackParameter::Pause => BoolModifier::Toggle,
			}),
			_ => None?,
		})
	}

	fn convert_seek(self) -> Option<AppEvent> {
		Some(match self.parameter? {
			CommandParameter::Time(TimeParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::SeekTo(value),
			CommandParameter::Time(TimeParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::SeekBy(TimeDelta::from_std(value).unwrap()),
			CommandParameter::Time(TimeParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::SeekBy(-TimeDelta::from_std(value).unwrap()),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::SeekToRelative(value),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::SeekByRelative(value),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::SeekByRelative(-value),
			_ => None?,
		})
	}

	fn convert_volume(self) -> Option<AppEvent> {
		Some(match self.parameter? {
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::ChangeVolumeTo((value * 100.).round() as u16),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::ChangeVolumeBy((value * 100.).round() as i16),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::ChangeVolumeBy(-(value * 100.).round() as i16),
			CommandParameter::Integer(IntegerParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::ChangeVolumeTo(value as u16),
			CommandParameter::Integer(IntegerParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::ChangeVolumeBy(value as i16),
			CommandParameter::Integer(IntegerParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::ChangeVolumeBy(-(value as i16)),
			_ => None?,
		})
	}

	fn convert_speed(self) -> Option<AppEvent> {
		Some(match self.parameter? {
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::ChangeSpeedTo((value * 100.).round() as u16),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::ChangeSpeedBy((value * 100.).round() as i16),
			CommandParameter::Percentage(PercentageParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::ChangeSpeedBy(-(value * 100.).round() as i16),
			CommandParameter::Integer(IntegerParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::ChangeSpeedTo(value as u16),
			CommandParameter::Integer(IntegerParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::ChangeSpeedBy(value as i16),
			CommandParameter::Integer(IntegerParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::ChangeSpeedBy(-(value as i16)),
			_ => None?,
		})
	}

	fn convert_timestamp(self) -> Option<AppEvent> {
		Some(match self.parameter? {
			CommandParameter::Time(TimeParameter {
				value,
				polarity: ParameterPolarity::Absolute,
			}) => AppEvent::ChangeTimestampTo(value),
			CommandParameter::Time(TimeParameter {
				value,
				polarity: ParameterPolarity::Positive,
			}) => AppEvent::ChangeTimestampBy(TimeDelta::from_std(value).unwrap()),
			CommandParameter::Time(TimeParameter {
				value,
				polarity: ParameterPolarity::Negative,
			}) => AppEvent::ChangeTimestampBy(-TimeDelta::from_std(value).unwrap()),
			_ => None?,
		})
	}
}

impl fmt::Display for Command {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(parameter) = self.parameter {
			write!(f, "{} {}", self.kind, parameter)
		} else {
			write!(f, "{}", self.kind)
		}
	}
}

impl str::FromStr for Command {
	type Err = ParseCommandError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Ok(kind) = s.parse() {
			Ok(Self {
				kind,
				parameter: None,
			})
		} else {
			let (kind_s, param_s) = s
				.split_once(' ')
				.ok_or(Self::Err::InvalidCommand(s.to_owned()))?;
			let kind: CommandKind = kind_s.parse()?;
			let parameter_result = match kind {
				CommandKind::Save => CommandParameter::from_str(param_s, &[]),
				CommandKind::Playback => {
					CommandParameter::from_str(param_s, &[CommandParameterKind::Playback])
				}
				CommandKind::Seek => CommandParameter::from_str(
					param_s,
					&[CommandParameterKind::Time, CommandParameterKind::Percentage],
				),
				CommandKind::Volume => CommandParameter::from_str(
					param_s,
					&[
						CommandParameterKind::Percentage,
						CommandParameterKind::Integer,
					],
				),
				CommandKind::Speed => CommandParameter::from_str(
					param_s,
					&[
						CommandParameterKind::Percentage,
						CommandParameterKind::Integer,
					],
				),
				CommandKind::Timestamp => {
					CommandParameter::from_str(param_s, &[CommandParameterKind::Time])
				}
			};
			let parameter =
				Some(
					parameter_result.map_err(|inner| Self::Err::InvalidParameter {
						cmd: s.to_owned(),
						inner,
					})?,
				);
			Ok(Self { kind, parameter })
		}
	}
}
