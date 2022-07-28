use std::io::Write;

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

    // TODO: More things that this command could do
    // - Create git repository
    // - Create simple templates for
    //      - index.html
    //      - content.html
    //      - blog.html
    let configuration_file_path = init_command.path();

    log::info!(
        "Initializing configuration in {:?}",
        configuration_file_path
    );

    let configuration = configuration::Config::init(configuration_file_path);
    create_content_directory(&configuration);
    creaet_templates_directory(&configuration);

    create_gitignore();
}

fn create_content_directory(config: &configuration::Config) {
    std::fs::create_dir_all(std::path::Path::new(&config.content_dir))
        .expect("Failed to create content directory");
}

fn creaet_templates_directory(config: &configuration::Config) {
    std::fs::create_dir_all(std::path::Path::new(&config.templates_directory))
        .expect("Failed to create build directory");
}

fn create_gitignore() {
    let mut gitignore_path = std::path::PathBuf::new();
    gitignore_path.push(".");
    gitignore_path.push(".gitgnore");

    if gitignore_path.exists() {
        log::debug!("Found .gitignore file, skipping creation.");
        return;
    }

    let gitignore_contents = String::from("just-html");

    let mut gitignore_file =
        std::fs::File::create(&gitignore_path).expect("failed to create a .gitignore file");
    gitignore_file
        .write_all(gitignore_contents.as_bytes())
        .expect("failed to write to .gitignore file")
}
