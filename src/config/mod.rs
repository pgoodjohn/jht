use std::path::PathBuf;

use crate::ConfigurationFilePath;

use super::configuration;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, arg_required_else_help = true)]
pub struct ConfigCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<ConfigCommands>,

    #[clap(short = 'f', long = "file", global = true)]
    configuration_file_path: Option<String>,
}

impl super::ConfigurationFilePath for ConfigCommand {
    fn configuration_file_path(&self) -> &Option<String> {
        &self.configuration_file_path
    }
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Validate the configuration
    Validate,
}

pub fn command(config_command: &ConfigCommand) {
    log::info!("Running config command");

    match config_command.command.as_ref() {
        Some(ConfigCommands::Validate) => {
            validate(config_command.path());
        }
        None => {
            log::info!("Unknown command")
        }
    }
}

fn validate(path: PathBuf) {
    log::info!("Validating config");
    let conf = configuration::Config::load(path);

    log::info!("{:?}", conf);
}
