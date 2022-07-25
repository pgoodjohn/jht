use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
extern crate dirs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub templates_dir: String,
    pub article_template: String,
    pub content_dir: String,
    pub build_config: BuildConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfig {
    pub build_directory: String,
    pub articles_directory: String,
    pub article_listings_page: String,
}

impl Config {
    pub fn load() -> Self {
        let config_location = ConfigLocation::new();

        println!("Loading config from: {:?}", config_location);

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

        // debug!("Config text loaded:\n\n{}", contents);

        let config: Config = toml::from_str(&contents).expect("Failed to parse configuration");

        println!("Loaded config: {:?}", config);

        config
    }

    pub fn init() -> Self {
        let config_location = ConfigLocation::new();

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

        let config = Config {
            build_config: BuildConfig {
                build_directory: String::from("./build"),
                articles_directory: String::from("./build/articles"),
                article_listings_page: String::from("articles"),
            },
            templates_dir: String::from("./templates"),
            article_template: String::from("./templates/article.html"),
            content_dir: String::from("./content"),
        };

        println!("{:?}", config);

        std::fs::write(
            &config_location.config_file_path,
            toml::to_string(&config).expect("Failed to serialize default config"),
        )
        .expect("Failed to write default config to file.");

        config
    }
}

static CONFIG_FILENAME: &str = "conf.toml";

#[derive(Debug)]
struct ConfigLocation {
    /// Full path of the configuration directory
    config_directory: PathBuf,
    /// Full path of the configuration file
    config_file_path: PathBuf,
}

impl ConfigLocation {
    pub fn new() -> Self {
        let mut config_path = PathBuf::new();

        if cfg!(debug_assertions) {
            config_path.push("/tmp/.just-html/");
        } else {
            config_path.push(std::env::current_dir().expect("Could not load current path"));
        }

        // 🤢

        let config_dir = config_path.clone();

        config_path.push(String::from(CONFIG_FILENAME));

        let c = ConfigLocation {
            config_directory: config_dir,
            config_file_path: config_path,
        };

        c
    }
}
