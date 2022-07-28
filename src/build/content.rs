use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::utils;

pub struct ContentList {
    pub items: Vec<String>,
}

pub fn build_content_pages(
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

    let content_template = load_content_template(content_page_template);

    // TODO: Add frontmatter support
    // See maybe: https://github.com/r7kamura/fronma
    for entry in content_directory_contents.into_iter() {
        let content_file = entry.unwrap();

        log::debug!("Building file {:?}", &content_file.path());

        if utils::is_markdown(&content_file.path()) {
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

fn load_content_template(content_page_template_path: &Path) -> String {
    match std::fs::read_to_string(content_page_template_path) {
        Ok(t) => {
            // TODO: Validate template
            validate_content_template(&t).expect("Invalid content template specified");
            // return template string
            t
        }
        Err(_e) => panic!("Could not load content page template"),
    }
}

fn validate_content_template(template: &String) -> Result<(), ()> {
    if template.contains("{content}") {
        Ok(())
    } else {
        Err(())
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
        content_page_template.replace("{content}", &convert_markdown_to_html(content_file_content));
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
