use clap::{Parser, Subcommand};
/// ---
/// Next steps:
/// - Init Script
///     - Create basic empty HTMLs (index.html, blog.html, content.html)
/// ----
mod build;
mod config;
mod configuration;
mod init;
mod logger;
mod serve;
mod utils;

/// JHT is a simple and straight forward static site generator
/// Bring your own HTML and fill it with some markdown content
#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(short = 'f', long = "file", global = true)]
    configuration_file_path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise Configuration
    Init(init::InitCommand),
    /// Build your website
    Build(build::BuildCommand),
    /// Manage your configuration
    Config(config::ConfigCommand),
    /// Serve your website from the build directory
    Serve(serve::ServeCommand),
}

fn main() {
    let cli = Cli::parse();

    logger::init(cli.debug);

    let configuration_file_path = cli.path();

    match cli.command {
        Some(Commands::Init(command)) => {
            // init_command();
            init::command(&command);
        }
        Some(Commands::Build(command)) => {
            let config = configuration::Config::load(configuration_file_path.to_path_buf());
            build::command(&command, &config);
        }
        Some(Commands::Config(command)) => {
            config::command(&command);
        }
        Some(Commands::Serve(command)) => {
            let config = configuration::Config::load(configuration_file_path.to_path_buf());
            serve::command(&command, &config);
        }
        None => {
            failure_message(); // Note that this will be handled by clap
        }
    }

    return;
}

fn failure_message() {
    log::info!("No command given");
}

pub trait ConfigurationFilePath {
    fn configuration_file_path(&self) -> &Option<String>;

    fn path(self: &Self) -> std::path::PathBuf {
        match &self.configuration_file_path() {
            Some(path) => {
                std::path::Path::new(&path).to_owned()
                // TODO: Validate that it always ends in .toml
            }
            None => std::path::Path::new("./config.toml").to_owned(),
        }
    }
}

impl ConfigurationFilePath for Cli {
    fn configuration_file_path(&self) -> &Option<String> {
        &self.configuration_file_path
    }
}
