use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub templates_directory: String,
    pub content_template: String,
    pub content_dir: String,
    pub build_config: BuildConfig,
    pub development_config: DevelopmentConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfig {
    pub build_directory: String,
    pub content_directory: String,
    pub content_listing_page: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DevelopmentConfig {
    pub port: u16,
}

impl Config {
    pub fn load(configuration_file_path: PathBuf) -> Self {
        let config_location = ConfigLocation::new(configuration_file_path);

        log::debug!("Loading config from: {:?}", config_location);

        match config_location.config_directory.exists() {
            true => {}
            false => {
                panic!("Could not find config directory, please run just-html init to initialise the configuration")
            }
        }

        match config_location.config_file_path.exists() {
            true => {}
            false => {
                panic!("Could not find config directory, please run just-html init to initialise the configuration")
            }
        }

        let contents = std::fs::read_to_string(config_location.config_file_path)
            .expect("Could not load configuration file");

        let config: Config = toml::from_str(&contents).expect("Failed to parse configuration");

        log::debug!("Loaded config: {:?}", config);

        config
    }

    pub fn init(configuration_file_path: PathBuf) -> Self {
        let config_location = ConfigLocation::new(configuration_file_path);

        match config_location.config_directory.exists() {
            true => {}
            false => {
                std::fs::create_dir_all(&config_location.config_directory)
                    .expect("Failed to create config directory");
            }
        }

        match config_location.config_file_path.exists() {
            true => {
                todo!("Already found a config file! Add option to override it");
            }
            false => {
                // panic!("Could not find config directory, please run just-html init to initialise the configuration")
                std::fs::File::create(&config_location.config_file_path)
                    .expect("Failed creating config file");
            }
        }

        // Default configuration
        let config = Config {
            build_config: BuildConfig {
                build_directory: String::from("./build"),
                content_directory: String::from("./build/blog"),
                content_listing_page: String::from("blog"),
            },
            development_config: DevelopmentConfig { port: 9999 },
            templates_directory: String::from("./templates"),
            content_template: String::from("./templates/content.html"),
            content_dir: String::from("./content"),
        };

        log::debug!("Created new config from default: {:?}", config);

        std::fs::write(
            &config_location.config_file_path,
            toml::to_string(&config).expect("Failed to serialize default config"),
        )
        .expect("Failed to write default config to file.");

        config
    }
}

#[derive(Debug)]
struct ConfigLocation {
    /// Full path of the configuration directory
    config_directory: PathBuf,
    /// Full path of the configuration file
    config_file_path: PathBuf,
}

impl ConfigLocation {
    pub fn new(input_config_path: PathBuf) -> Self {
        log::debug!("Initializing ConfigLocation for {:?}", input_config_path);

        let mut config_dir = input_config_path.clone();
        config_dir.pop();

        let c = ConfigLocation {
            config_directory: config_dir,
            config_file_path: input_config_path,
        };

        c
    }
}
