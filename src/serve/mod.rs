use crate::configuration::Config;
use clap::Parser;

#[derive(Parser)]
pub struct ServeCommand {
    #[clap(short, long, global = true)]
    debug: bool,

    #[clap(short, long)]
    port: Option<u16>,
}
#[tokio::main]
pub async fn command(command: &ServeCommand, config: &Config) {
    let build_directory = warp::fs::dir(String::from(&config.build_config.build_directory));

    let port = get_port(command, config);

    warp::serve(build_directory).run(([0, 0, 0, 0], port)).await;
}

fn get_port(command: &ServeCommand, config: &Config) -> u16 {
    match command.port {
        Some(port) => port,
        None => config.development_config.port,
    }
}
