use core::{fmt, str, time::Duration};

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use strum::EnumDiscriminants;

use super::error::*;

#[derive(
	Debug, Clone, Copy, PartialEq, derive_more::Display, EnumDiscriminants, Serialize, Deserialize,
)]
#[serde(untagged)]
#[strum_discriminants(name(CommandParameterKind))]
pub enum CommandParameter {
	Playback(PlaybackParameter),
	Integer(IntegerParameter),
	Percentage(PercentageParameter),
	Time(TimeParameter),
}

impl CommandParameter {
	pub fn from_str(
		s: &str,
		allowed_kinds: &[CommandParameterKind],
	) -> Result<Self, ParseCommandParameterError> {
		if allowed_kinds.is_empty() {
			Err(ParseCommandParameterError::NoAllowedParameters)?;
		}
		for &kind in allowed_kinds {
			let result = match kind {
				CommandParameterKind::Playback => {
					s.parse().map(CommandParameter::Playback).map_err(|_| ())
				}
				CommandParameterKind::Integer => {
					s.parse().map(CommandParameter::Integer).map_err(|_| ())
				}
				CommandParameterKind::Percentage => {
					s.parse().map(CommandParameter::Percentage).map_err(|_| ())
				}
				CommandParameterKind::Time => s.parse().map(CommandParameter::Time).map_err(|_| ()),
			};
			if let Ok(value) = result {
				return Ok(value);
			}
		}

		Err(ParseCommandParameterError::InvalidParameter(s.to_owned()))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlaybackParameter {
	#[serde(alias = "start", alias = "continue", alias = "unpause")]
	Play,
	#[serde(alias = "stop")]
	Pause,
	Toggle,
}

impl fmt::Display for PlaybackParameter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Play => write!(f, "play"),
			Self::Pause => write!(f, "pause"),
			Self::Toggle => write!(f, "toggle"),
		}
	}
}

impl str::FromStr for PlaybackParameter {
	type Err = ParsePlaybackParameterError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"play" | "start" => Ok(Self::Play),
			"pause" | "stop" => Ok(Self::Pause),
			"toggle" => Ok(Self::Toggle),
			_ => Err(Self::Err::InvalidKeyword(s.to_owned())),
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
pub struct IntegerParameter {
	pub value: u32,
	pub polarity: ParameterPolarity,
}

impl fmt::Display for IntegerParameter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}", self.polarity, self.value)
	}
}

impl str::FromStr for IntegerParameter {
	type Err = ParseIntegerParameterError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(c) = s.chars().next()
			&& let Ok(polarity) = ParameterPolarity::try_from(c)
		{
			let value = s[c.len_utf8()..].parse()?;
			Ok(Self { value, polarity })
		} else {
			let value = s.parse()?;
			Ok(Self {
				value,
				..Default::default()
			})
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, SerializeDisplay, DeserializeFromStr)]
pub struct PercentageParameter {
	pub value: f32,
	pub polarity: ParameterPolarity,
}

impl fmt::Display for PercentageParameter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}{}%", self.polarity, self.value * 100.)
	}
}

impl str::FromStr for PercentageParameter {
	type Err = ParsePercentageParameterError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let sign_idx = s
			.char_indices()
			.last()
			.filter(|(_, c)| *c == '%')
			.ok_or(Self::Err::MissingPercentSign)?
			.0;

		if let Some(c) = s.chars().next()
			&& let Ok(polarity) = ParameterPolarity::try_from(c)
		{
			let value = s[c.len_utf8()..sign_idx].parse()?;
			Ok(Self { value, polarity })
		} else {
			let value = s[..sign_idx].parse()?;
			Ok(Self {
				value,
				..Default::default()
			})
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, SerializeDisplay, DeserializeFromStr)]
pub struct TimeParameter {
	pub value: Duration,
	pub polarity: ParameterPolarity,
}

impl fmt::Display for TimeParameter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.value == Duration::ZERO {
			write!(f, "{}0s", self.polarity)
		} else if self.value < Duration::from_secs(1) {
			write!(f, "{}{}ms", self.polarity, self.value.subsec_millis())
		} else if self.value < Duration::from_secs(60) && self.value.subsec_millis() == 0 {
			write!(f, "{}{}", self.polarity, self.value.as_secs())
		} else if self.value < Duration::from_secs(60) {
			write!(
				f,
				"{}{}.{:0>2}",
				self.polarity,
				self.value.as_secs(),
				self.value.subsec_millis()
			)
		} else if self.value.subsec_millis() == 0 {
			write!(
				f,
				"{}{}:{:0>2}",
				self.polarity,
				self.value.as_secs() / 60,
				self.value.as_secs(),
			)
		} else {
			write!(
				f,
				"{}{}:{:0>2}.{:0>2}",
				self.polarity,
				self.value.as_secs() / 60,
				self.value.as_secs(),
				self.value.subsec_millis(),
			)
		}
	}
}

impl str::FromStr for TimeParameter {
	type Err = ParseTimeParameterError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (polarity, s) = if let Some(c) = s.chars().next()
			&& let Ok(polarity) = ParameterPolarity::try_from(c)
		{
			(polarity, &s[c.len_utf8()..])
		} else {
			(Default::default(), s)
		};

		if let Some((s, unit)) = s.split_at_checked(s.len().saturating_sub(2))
			&& unit == "ms"
		{
			let value = Duration::from_millis(s.parse()?);
			Ok(Self { value, polarity })
		} else if let Some((s, unit)) = s.split_at_checked(s.len().saturating_sub(1))
			&& unit == "s"
		{
			let value = Duration::from_secs_f32(s.parse()?);
			Ok(Self { value, polarity })
		} else if let Some((min, sec)) = s.split_once(':') {
			let value = Duration::from_secs_f32(min.parse::<f32>()? * 60. + sec.parse::<f32>()?);
			Ok(Self { value, polarity })
		} else {
			Err(Self::Err::InvalidFormat(s.to_owned()))
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ParameterPolarity {
	#[default]
	Absolute,
	Positive,
	Negative,
}

impl fmt::Display for ParameterPolarity {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Absolute => write!(f, "="),
			Self::Positive => write!(f, "+"),
			Self::Negative => write!(f, "-"),
		}
	}
}

impl TryFrom<char> for ParameterPolarity {
	type Error = ParseParameterPolarityError;

	fn try_from(value: char) -> Result<Self, Self::Error> {
		match value {
			'=' => Ok(Self::Absolute),
			'+' => Ok(Self::Positive),
			'-' => Ok(Self::Negative),
			c => Err(ParseParameterPolarityError(c)),
		}
	}
}
