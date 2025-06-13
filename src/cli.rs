use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// A tui application for synchronising lyrics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	pub path: Option<PathBuf>,

	#[arg(long)]
	pub print_config: Option<Option<ConfigFiletype>>,
}

#[derive(ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFiletype {
	#[default]
	Toml,
	Json,
	Yaml,
}
