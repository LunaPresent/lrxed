use ratatui::{
	layout::{Constraint, Layout},
	prelude::{Buffer, Rect},
	style::Style,
	text::Span,
	widgets::{LineGauge, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeWidget {
	volume: f64,
	unfilled_style: Style,
	filled_style: Style,
}

impl VolumeWidget {
	pub fn new(volume: impl Into<f64>) -> Self {
		Self {
			volume: volume.into(),
			unfilled_style: Default::default(),
			filled_style: Default::default(),
		}
	}

	pub fn unfilled_style(mut self, style: impl Into<Style>) -> Self {
		self.unfilled_style = style.into();
		self
	}

	pub fn filled_style(mut self, style: impl Into<Style>) -> Self {
		self.filled_style = style.into();
		self
	}
}

impl Widget for VolumeWidget {
	fn render(self, area: Rect, buf: &mut Buffer)
	where
		Self: Sized,
	{
		let [symbol_area, bar_area, label_area] = Layout::horizontal([
			Constraint::Length(1),
			Constraint::Min(2),
			Constraint::Length(5),
		])
		.areas(area);

		let symbol = if self.volume >= 0.75 {
			""
		} else if self.volume >= 0.25 {
			""
		} else if self.volume > 0.00 {
			""
		} else {
			""
		};

		Span::from(symbol).render(symbol_area, buf);

		let bar_width = (bar_area.width - 1) as f64;
		let bar = LineGauge::default()
			.ratio((self.volume + 1. / (2. * bar_width) + 1.0e-6).min(1.))
			.unfilled_style(self.unfilled_style)
			.filled_style(self.filled_style)
			.label("");
		bar.render(bar_area, buf);

		Span::from(format!(" {}%", (self.volume * 100.).round() as u16)).render(label_area, buf);
	}
}
