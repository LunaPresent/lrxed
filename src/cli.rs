use std::path::PathBuf;

use clap::Parser;

/// A tui application for synchronising lyrics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	pub path: Option<PathBuf>,
}
