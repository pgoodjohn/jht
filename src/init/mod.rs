use super::configuration;
use crate::ConfigurationFilePath;
use clap::Parser;
use log;

#[derive(Parser)]
#[clap(version, about)]
pub struct InitCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(short = 'f', long = "file", global = true)]
    configuration_file_path: Option<String>,
}

impl super::ConfigurationFilePath for InitCommand {
    fn configuration_file_path(&self) -> &Option<String> {
        &self.configuration_file_path
    }
}

pub fn command(init_command: &InitCommand) {
    log::info!("Running init command");

    let configuration_file_path = init_command.path();

    log::info!(
        "Initializing configuration in {:?}",
        configuration_file_path
    );

    // Create configuration
    let configuration = configuration::Config::init(configuration_file_path);
    // Create folder for content
    std::fs::create_dir_all(std::path::Path::new(
        &configuration.build_config.build_directory,
    ))
    .expect("Failed to create build directory");
    // Create folder for templates
    std::fs::create_dir_all(std::path::Path::new(&configuration.templates_directory))
        .expect("Failed to create build directory");
}
