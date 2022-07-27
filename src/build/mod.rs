use super::configuration;
use clap::Parser;
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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
    let content_list = build_content_pages(
        std::path::Path::new("./templates/article.html"),
        std::path::Path::new(&config.build_config.articles_directory),
        std::path::Path::new(&config.content_dir),
    );

    // Build content list page
    build_listing_page(
        content_list,
        std::path::Path::new(&config.templates_directory),
        std::path::Path::new(&config.build_config.build_directory),
        &config.build_config.article_listings_page,
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

struct ContentList {
    items: Vec<String>,
}

fn build_content_pages(
    content_page_template: &Path,
    content_build_directory: &Path,
    content_directory: &Path,
) -> ContentList {
    log::info!(
        "Building content pages with template {}",
        content_page_template.as_os_str().to_str().unwrap()
    );

    // TODO: This should not be specified in the config but should be a combination of two config value
    // build directory + (new) content page naming (or something like that)
    // content page naming would refer to:
    // - listing template name
    // - path of content pages
    // - maybe something else
    create_content_build_folder_if_it_does_not_exist(content_build_directory);

    build_content_files(
        content_directory,
        content_build_directory,
        content_page_template,
    )
}

fn build_content_files(
    content_directory: &Path,
    content_build_directory: &Path,
    content_page_template: &Path,
) -> ContentList {
    let content_directory_contents = std::fs::read_dir(content_directory)
        .expect("Could not read contents of contents directory");

    let mut content_pages = Vec::new();

    // TODO: validate that the content template has {article} in it
    let content_template =
        std::fs::read_to_string(content_page_template).expect("article template missing");

    // TODO: Add frontmatter support
    for entry in content_directory_contents.into_iter() {
        let content_file = entry.unwrap();

        log::debug!("Building file {:?}", &content_file.path());

        if is_markdown(&content_file.path()) {
            log::debug!("Markdown file detected, converting to html");
            let built_content_file = create_content_file_from_markdown_and_html_template(
                &content_file.path(),
                content_template.clone(),
                content_build_directory,
            );

            content_pages.push(built_content_file.file_name);
        }
    }

    log::info!("Built content pages");

    for content_page in &content_pages {
        log::info!("{}", content_page);
    }

    ContentList {
        items: content_pages,
    }
}

fn create_content_build_folder_if_it_does_not_exist(content_folder_path: &Path) {
    match content_folder_path.exists() {
        true => {}
        false => {
            log::debug!(
                "Creating {}",
                content_folder_path
                    .to_str()
                    .expect("Could not unwrap content folder path")
            );
            std::fs::create_dir(content_folder_path)
                .expect("Failed to create content build folder ");
        }
    }
}

fn create_content_file_from_markdown_and_html_template(
    markdown_file: &Path,
    content_page_template: String,
    content_build_directory: &Path,
) -> BuiltContentFile {
    let content_file_content = std::fs::read_to_string(markdown_file).expect("unable to read file");

    let built_content_file = BuiltContentFile::new(content_build_directory, &markdown_file);

    let prepared_template =
        content_page_template.replace("{article}", &convert_markdown_to_html(content_file_content));
    write_content_to_file(&built_content_file, &prepared_template);

    built_content_file
}

fn convert_markdown_to_html(markdown_content: String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = MarkdownParser::new_ext(&markdown_content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

fn write_content_to_file(file_path: &BuiltContentFile, contents: &String) {
    let mut new_file =
        File::create(&file_path.path).expect("failed to create a file to store the article");

    new_file
        .write_all(contents.as_bytes())
        .expect("unable to write to article page");
}

struct BuiltContentFile {
    path: PathBuf,
    file_name: String,
}

impl BuiltContentFile {
    pub fn new(build_directory: &Path, content_file_path: &Path) -> Self {
        let mut path = std::path::PathBuf::new();

        let formatted_path = String::from(format!(
            "{}/{}.html",
            build_directory
                .as_os_str()
                .to_str()
                .expect("Could not convert articles build directory in new file path"),
            content_file_path
                .file_stem()
                .expect("Unable to get file stem from article file path")
                .to_str()
                .expect("Unable to convert file stem from article file path to string"),
        ));

        path.push(std::path::Path::new(&formatted_path));

        Self {
            path: path,
            file_name: formatted_path,
        }
    }
}

fn build_listing_page(
    content_list: ContentList,
    templates_directory: &Path,
    build_directory: &Path,
    article_listing_page_name: &String,
) -> Result<(), ()> {
    log::info!("Adding {:?} to listing page", content_list.items);

    let mut article_hrefs = String::new();

    for article in content_list.items {
        article_hrefs.push_str(&format!(
            "<a href={}>{}</a> <br />",
            // TODO Change this to strip based on build_directory filepath
            article.strip_prefix("./build/").expect("big bad"),
            article.strip_prefix("./build/").expect("big bad")
        ));
    }

    let mut z = String::from(article_listing_page_name);
    z.push_str(".html");
    let mut x = templates_directory.clone().to_path_buf();

    x.push(std::path::Path::new(&z));

    let list_template = std::fs::read_to_string(&x).expect("listing templates missing");

    let list_page = list_template.replace("{article_list}", &article_hrefs);

    let mut y = build_directory.clone().to_path_buf();

    y.push(std::path::Path::new(&z));
    let mut new_article_list = File::create(y).expect("unable to create new listing page");

    new_article_list
        .write_all(list_page.as_bytes())
        .expect("could not create new listing file");

    Ok(())
}

fn build_stylesheets(templates_directory: &Path, build_directory: &Path) {
    log::info!("Building stylesheets");

    let all_templates = std::fs::read_dir(templates_directory)
        .expect("Failed reading templates in templates directory");

    let stylesheets = all_templates.filter(|x| is_stylesheet(x));

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

fn is_markdown(path: &Path) -> bool {
    match path.extension() {
        None => false,
        Some(extension) => match extension.to_str() {
            Some("md") => true,
            _ => false,
        },
    }
}

fn is_stylesheet(entry: &Result<std::fs::DirEntry, std::io::Error>) -> bool {
    match entry {
        Ok(entry) => match entry.path().extension() {
            None => false,
            Some(extension) => match extension.to_str() {
                Some("css") => true,
                _ => false,
            },
        },
        Err(_e) => false,
    }
}
