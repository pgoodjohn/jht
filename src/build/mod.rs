use super::configuration;
use super::utils;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod content;

#[derive(Parser)]
pub struct BuildCommand {
    #[clap(short, long, global = true)]
    debug: bool,
}

pub fn command(_command: &BuildCommand, config: &configuration::Config) {
    log::info!("Starting build process");

    create_build_directory(std::path::Path::new(&config.build_config.build_directory));

    // Build index.html
    build_index(
        &config.templates_directory,
        &config.build_config.build_directory,
    )
    .expect("Could not build index.html");

    // Build content pages
    let content_list = content::build_content_pages(
        std::path::Path::new(&config.content_template),
        std::path::Path::new(&config.build_config.content_directory),
        std::path::Path::new(&config.content_dir),
    );

    // Build content list page
    build_listing_page(
        content_list,
        std::path::Path::new(&config.templates_directory),
        std::path::Path::new(&config.build_config.build_directory),
        &&config.build_config.content_listing_page,
    )
    .expect("Could not build listing page");

    // Build other pages
    build_stylesheets(
        std::path::Path::new(&config.templates_directory),
        std::path::Path::new(&config.build_config.build_directory),
    );
}

fn create_build_directory(build_directory_path: &Path) {
    match build_directory_path.exists() {
        true => {}
        false => {
            log::debug!("Creating empty build directory");
            std::fs::create_dir_all(build_directory_path)
                .expect("Failed to create build directory");
        }
    }
}

fn build_index(templates_directory: &String, build_directory: &String) -> Result<(), ()> {
    log::info!(
        "Creating index.html from {}/index.html",
        templates_directory
    );

    let x = format!("{}/index.html", templates_directory);
    let index_template =
        std::fs::read_to_string(std::path::Path::new(&x)).expect("index.html template missing");

    let y = format!("{}/index.html", build_directory);
    let mut new_index_page =
        File::create(std::path::Path::new(&y)).expect("Failed creating index in build directory");

    new_index_page
        .write_all(index_template.as_bytes())
        .expect("Failed writing to build/index.html");
    Ok(())
}

fn build_listing_page(
    content_list: content::ContentList,
    templates_directory: &Path,
    build_directory: &Path,
    content_listing_page_name: &String,
) -> Result<(), ()> {
    log::info!("Adding {:?} to listing page", content_list.items);

    let mut content_hrefs = String::new();

    // TODO: The HTML from this should come from a template.
    for content in content_list.items {
        content_hrefs.push_str(&format!(
            "<a href={}>{}</a> <br />",
            // TODO Change this to strip based on build_directory filepath
            content.strip_prefix("./build/").expect("big bad"),
            content.strip_prefix("./build/").expect("big bad")
        ));
    }

    let mut z = String::from(content_listing_page_name);
    z.push_str(".html");
    let mut x = templates_directory.clone().to_path_buf();

    x.push(std::path::Path::new(&z));

    let list_template = std::fs::read_to_string(&x).expect("listing templates missing");

    let list_page = list_template.replace("{content_list}", &content_hrefs);

    let mut y = build_directory.clone().to_path_buf();

    y.push(std::path::Path::new(&z));
    let mut new_content_list = File::create(y).expect("unable to create new listing page");

    new_content_list
        .write_all(list_page.as_bytes())
        .expect("could not create new listing file");

    Ok(())
}

fn build_stylesheets(templates_directory: &Path, build_directory: &Path) {
    log::info!("Building stylesheets");

    let all_templates = std::fs::read_dir(templates_directory)
        .expect("Failed reading templates in templates directory");

    let stylesheets = all_templates.filter(|x| {
        if let Ok(template_path) = x {
            utils::is_stylesheet(&template_path.path())
        } else {
            false
        }
    });

    for stylesheet in stylesheets {
        match stylesheet {
            Ok(template_file) => {
                log::debug!("Building {:?}", template_file.path());
                // Maybe do some minimization here
                let mut built_file = build_directory.to_path_buf();
                built_file.push(template_file.file_name());

                std::fs::copy(template_file.path(), built_file)
                    .expect("Could not copy stylesheet to build directory");
            }
            Err(_e) => {}
        }
    }
}
