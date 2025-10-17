use std::io::{Stdout, stdout};

use crossterm::cursor;
use crossterm::event::{
	DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use derive_more::{Deref, DerefMut};
use ratatui::backend::CrosstermBackend as Backend;

#[derive(Debug, Deref, DerefMut)]
pub struct Terminal(ratatui::Terminal<Backend<Stdout>>);

impl Terminal {
	pub fn new() -> eyre::Result<Self> {
		Ok(Self(ratatui::Terminal::new(Backend::new(stdout()))?))
	}

	pub fn enter(&mut self) -> eyre::Result<()> {
		let hook = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info| {
			let _ = Self::restore();
			hook(info);
		}));

		Self::init()?;
		Ok(())
	}

	pub fn exit(&mut self) -> eyre::Result<()> {
		if crossterm::terminal::is_raw_mode_enabled()? {
			self.flush()?;
			Self::restore()?;
		}
		Ok(())
	}

	pub fn suspend(&mut self) -> eyre::Result<()> {
		self.exit()?;
		#[cfg(not(windows))]
		signal_hook::low_level::raise(signal_hook::consts::signal::SIGTSTP)?;
		Ok(())
	}

	pub fn resume(&mut self) -> eyre::Result<()> {
		self.enter()?;
		Ok(())
	}

	fn init() -> eyre::Result<()> {
		crossterm::terminal::enable_raw_mode()?;
		crossterm::execute!(stdout(), EnterAlternateScreen, cursor::Hide)?;
		crossterm::execute!(stdout(), EnableMouseCapture)?;
		crossterm::execute!(stdout(), EnableBracketedPaste)?;
		Ok(())
	}

	fn restore() -> eyre::Result<()> {
		crossterm::terminal::disable_raw_mode()?;
		crossterm::execute!(stdout(), DisableBracketedPaste)?;
		crossterm::execute!(stdout(), DisableMouseCapture)?;
		crossterm::execute!(stdout(), LeaveAlternateScreen, cursor::Show)?;
		Ok(())
	}
}

impl Drop for Terminal {
	fn drop(&mut self) {
		self.exit().unwrap();
	}
}
