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
        path: Option<String>,
    },
    /// Validate the configuration
    Validate {
        #[clap(short, long)]
        path: Option<String>,
    },
}

pub fn command(config_command: &ConfigCommand) {
    println!("Config command");

    match config_command.command.as_ref() {
        Some(ConfigCommands::Init { path }) => {
            initialize();
        }
        Some(ConfigCommands::Validate { path }) => {
            validate();
        }
        None => {
            println!("Smth else")
        }
    }
}

fn initialize() {
    println!("Initialising config");

    configuration::Config::init();
}

fn validate() {
    println!("Validating config");
    let conf = configuration::Config::load();

    println!("{:?}", conf);
}
