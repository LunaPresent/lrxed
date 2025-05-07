use std::{str::FromStr, time::Duration};

use color_eyre::eyre::{self, OptionExt};

#[derive(Debug, Default, Clone)]
pub struct Timestamp {
	time: Duration,
	text: String,
}

impl Timestamp {
	pub fn time(&self) -> Duration {
		self.time
	}

	pub fn text(&self) -> &str {
		&self.text
	}

	fn format_time(m: u64, s: u64, c: u32) -> String {
		format!("{m:0>2}:{s:0>2}.{c:0>2}")
	}
}

impl From<Duration> for Timestamp {
	fn from(time: Duration) -> Self {
		let centis = time.subsec_millis() / 10;
		Timestamp {
			time: Duration::new(time.as_secs(), centis * 10_000_000),
			text: Self::format_time(time.as_secs() / 60, time.as_secs() % 60, centis),
		}
	}
}

impl FromStr for Timestamp {
	type Err = eyre::ErrReport;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (mm, s) = s.split_once(':').ok_or_eyre("Invalid timestamp format")?;
		let (ss, cc) = s.split_once('.').ok_or_eyre("Invalid timestamp format")?;
		let minutes = mm.trim().parse::<u64>()?;
		let seconds = ss.trim().parse::<u64>()?;
		let centis = cc.trim().parse::<u32>()?;
		if seconds >= 60 || centis >= 100 {
			return Err(eyre::eyre!("Value out of range"));
		};
		let time = Duration::new(minutes * 60 + seconds, centis * 10_000_000);
		let text = Self::format_time(minutes, seconds, centis);
		Ok(Timestamp { time, text })
	}
}

impl PartialEq for Timestamp {
	fn eq(&self, other: &Self) -> bool {
		self.time == other.time
	}
}

impl Eq for Timestamp {}

impl PartialOrd for Timestamp {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		self.time.partial_cmp(&other.time)
	}
}

impl Ord for Timestamp {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.time.cmp(&other.time)
	}
}
