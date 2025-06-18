use ratatui::{
	prelude::{Buffer, Rect},
	style::Style,
	widgets::{Block, BorderType, Paragraph, Widget, Wrap},
};

#[derive(Debug, Clone, PartialEq)]
pub struct ToastWidget<'a> {
	paragraph: Paragraph<'a>,
	border_style: Style,
}

impl<'a> ToastWidget<'a> {
	pub fn new(paragraph: Paragraph<'a>) -> Self {
		Self {
			paragraph,
			border_style: Default::default(),
		}
	}

	pub fn border_style<S: Into<Style>>(mut self, style: S) -> Self {
		self.border_style = style.into();
		self
	}
}

impl<'a> Widget for ToastWidget<'a> {
	fn render(self, area: Rect, buf: &mut Buffer)
	where
		Self: Sized,
	{
		let block = Block::bordered()
			.border_type(BorderType::Plain)
			.border_style(self.border_style)
			.title_top("Error");
		let inner = block.inner(area);
		block.render(area, buf);

		self.paragraph
			.centered()
			.wrap(Wrap { trim: true })
			.render(inner, buf);
	}
}
