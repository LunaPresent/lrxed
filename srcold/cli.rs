use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// A tui application for synchronising lyrics
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	pub path: Option<PathBuf>,

	#[arg(long, value_name = "PATH")]
	/// path to config file to use instead of the default location
	pub config: Option<PathBuf>,
	#[arg(long, value_name = "FORMAT")]
	/// print the user config, or the defaults for unset values, then exit
	pub print_config: Option<Option<ConfigFiletype>>,
	/// print the location of the user config, then exit
	#[arg(long, value_name = "FORMAT")]
	pub print_config_path: Option<Option<ConfigFiletype>>,
}

#[derive(ValueEnum, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFiletype {
	#[default]
	Toml,
	Json,
	Yaml,
}
