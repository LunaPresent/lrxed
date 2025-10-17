mod cli;
mod command;
mod config;
mod event;
mod util;

use color_eyre::eyre;

#[tokio::main]
async fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	let cli = cli::Cli::new();
	oprabeli::app::App::new()
		.with_tick_interval(core::time::Duration::from_secs_f64(0.25))
		.with_frame_interval(core::time::Duration::from_secs_f64(1. / 144.))
		// .with_entity(|e| {
		// 	e.with_component(component::ErrorReporterComponent::new())?
		// 		.with_component(config::ConfigManager::new(cli.config_path()))?
		// 		.with_component(component::RootComponent::default())
		// })?
		// .with_entity(|e| {
		// 	e.with_component(component::FpsComponent::new(util::OctDirection::UpRight))
		// })?
		.run()
		.await
}
