use crate::configuration::Config;

#[tokio::main]
pub async fn command(config: Config) {
    let build_directory = warp::fs::dir(config.build_config.build_directory);

    warp::serve(build_directory).run(([0, 0, 0, 0], 3030)).await;
}
