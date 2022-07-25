use super::configuration;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about)]
pub struct ConfigCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<ConfigCommands>,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Initialize the configuration
    Init {
        #[clap(short, long)]
        _path: Option<String>,
    },
    /// Validate the configuration
    Validate {
        #[clap(short, long)]
        _path: Option<String>,
    },
}

pub fn command(config_command: &ConfigCommand) {
    log::info!("Running config command");

    match config_command.command.as_ref() {
        Some(ConfigCommands::Init { _path }) => {
            initialize();
        }
        Some(ConfigCommands::Validate { _path }) => {
            validate();
        }
        None => {
            log::info!("Unknown command")
        }
    }
}

fn initialize() {
    log::info!("Initialising config");

    configuration::Config::init();
}

fn validate() {
    log::info!("Validating config");
    let conf = configuration::Config::load();

    log::info!("{:?}", conf);
}
