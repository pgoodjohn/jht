use clap::{Parser, Subcommand};

mod build;
mod config;
mod configuration;

/// Next steps:
/// - Init Script (creates index.html, listing.html, config.toml)
/// - Config reading
/// - Styles
///

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise Configuration
    Init,
    /// Build your website
    Build(build::BuildCommand),
    /// Manage your configuration
    Config(config::ConfigCommand),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            init_command();
        }
        Some(Commands::Build(command)) => {
            let config = configuration::Config::load();
            build::command(&command, &config);
        }
        Some(Commands::Config(command)) => {
            config::command(&command);
        }
        None => {
            failure_message();
        }
    }

    return;
}

fn init_command() {
    println!("Init command");
}

fn failure_message() {
    println!("No command given");
}
