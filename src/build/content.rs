use lazy_static::lazy_static;
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use regex::Regex;
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

    for entry in content_directory_contents.into_iter() {
        let content_file = entry.unwrap();

        log::debug!("Building file {:?}", &content_file.path());

        if utils::is_markdown(&content_file.path()) {
            log::debug!("Markdown file detected, converting to html");

            let file = ContentFile::new(&content_file.path());
            let built_file = file.build(content_template.clone(), content_build_directory);

            content_pages.push(built_file.file_name);
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

struct ContentFile {
    _path: PathBuf,
    file_name: String,
    raw_contents: String,
    _frontmatter: Option<std::collections::HashMap<String, String>>,
}

impl ContentFile {
    pub fn new(path: &Path) -> Self {
        let file_contents = std::fs::read_to_string(path).expect("Unable to read Content file");

        let file_name = path.file_stem().expect("Unable to retrieve file stem from Content file").to_str().expect("Could not convert Content file stem to string");

        let parsed_content = ContentFileFrontmatterAndRawContent::from_file_contents(file_contents);

        ContentFile {
            _path: path.to_path_buf(),
            file_name: String::from(file_name),
            raw_contents: parsed_content.raw_content,
            _frontmatter: parsed_content.frontmatter
        } 
    }

    pub fn build(self: &Self, template: String, build_directory: &Path) -> BuiltContentFile {
        let built_content_file = BuiltContentFile::from_file_name(build_directory, &self.file_name);

        let mut prepared_template = template.replace("{content}", &convert_markdown_to_html(&self.raw_contents));

        // Find and replace any {key} with value from frontmatter if some frontmatter was in the
        // file.
        match &self._frontmatter {
            Some(frontmatter) => {
                for (key, value) in frontmatter.iter() {
                    log::debug!("Found frontmatter {:?}: {:?}", key, value );
                    
                    let formatted_key = format!(r#"{{{}}}"#, key);

                    log::debug!("Replacing key {:?} in template", formatted_key);

                    prepared_template = prepared_template.replace(formatted_key.as_str(), value);
                }
            },
            None => {},
        }
       
        write_content_to_file(&built_content_file, &prepared_template);

        built_content_file
    }
}

struct ContentFileFrontmatterAndRawContent {
    raw_content: String,
    frontmatter: Option<std::collections::HashMap<String, String>>
}

impl ContentFileFrontmatterAndRawContent {
    pub fn from_file_contents(file_contents: String) -> Self {

        let frontmatter = parse_frontmatter(&file_contents);
        let content_without_frontmatter = remove_frontmatter(file_contents);

        ContentFileFrontmatterAndRawContent {
            raw_content: content_without_frontmatter,
            frontmatter
        }
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

fn remove_frontmatter(file_content: String) -> String {
    lazy_static! {
        static ref FRONTMATTER_REGEX: Regex =
            Regex::new(r#"^---\n(?P<frontmatter>(.*:\s.*\n)*)---"#).unwrap();
    }

    let fixed_string = FRONTMATTER_REGEX.replace(&file_content, "").to_string();

    fixed_string
}

fn parse_frontmatter(content_file_content: &String) -> Option<std::collections::HashMap<String, String>> {
    // ---(?<frontmatter>(.|\n)*)---
    lazy_static! {
        static ref FRONTMATTER_REGEX: Regex =
             Regex::new(r#"^---\n(?P<frontmatter>(.*:\s.*\n)*)---"#).unwrap();
    }

    let locs = FRONTMATTER_REGEX.captures(&content_file_content);

    match locs {
        None =>  {
            log::debug!("No frontmatter detected");
            return None;
        },
        Some(captures) => {
            let frontmatter_text = captures
                .name("frontmatter")
                .expect("could not get frontmatter from text")
                .as_str();

            log::debug!("Found frontmatter {:?}", &frontmatter_text);

            return parse_key_value_pairs(&frontmatter_text);
        }
    }
}

#[cfg(test)]
mod test_frontmatter {
    use super::parse_frontmatter;

    #[test]
    fn it_parses_a_single_frontmatter_at_the_top_of_the_file() {
        let input = r#"---
marco: polo
leonardo: da vinci
---
"#;

        let parsed_frontmatter = parse_frontmatter(&String::from(input));

        match parsed_frontmatter {
            None => panic!("Parsing frontmatter returned nothing."),
            Some(frontmatter) => {
                assert_eq!(true, frontmatter.contains_key("marco"));
            }
        }

    }

    #[test]
    fn it_parses_a_single_frontmatter_at_the_top_of_the_file_if_multiple_triple_dashes_are_in_the_content() {
    let input = r#"---
marco: polo
leonardo: da vinci
---

Lorem ipsum dolorem sic amet and other things as such.

---


Even more content down here

---
"#;

        let parsed_frontmatter = parse_frontmatter(&String::from(input));

        match parsed_frontmatter {
            None => panic!("Parsing frontmatter returned nothing."),
            Some(frontmatter) => {
                assert_eq!(true, frontmatter.contains_key("marco"));
                assert_eq!(2, frontmatter.len());
            }
        }
    }
    
    #[test]
    fn it_only_parses_frontmatter_if_at_the_beginning_of_the_file() {
let input = r#"There is some more content in this file about stuff and maybe an example:

---
marco: polo
leonardo: da vinci
---

Lorem ipsum dolorem sic amet and other things as such.

---


Even more content down here

---
"#;

        let parsed_frontmatter = parse_frontmatter(&String::from(input));

        match parsed_frontmatter {
            Some (_f) => panic!("Parsing frontmatter returned nothing."),
            None => {},
        }

    }

}

fn parse_key_value_pairs(frontmatter: &str) -> Option<std::collections::HashMap<String, String>> {
    lazy_static! {
        static ref KEY_VALUE_REGEX: Regex = Regex::new("^(?P<key>.*):\\s(?P<value>.*)$").unwrap();
    }

    let mut key_values = std::collections::HashMap::<String, String>::new();

    for line in frontmatter.lines() {
        let locs = KEY_VALUE_REGEX.captures(line);

        match locs {
            None => {
                return None;
            }
            Some(captures) => {
                let key = captures
                    .name("key")
                    .expect("could not get key from text")
                    .as_str();

                let value = captures
                    .name("value")
                    .expect("could not get value from text")
                    .as_str();

                log::debug!("Parsed key: value pair {:?} {:?}", &key, &value);

                key_values.insert(String::from(key), String::from(value));
            }
        }
    }

    log::debug!("Parsed key value struct: {:?}", key_values);

    Some(key_values)
}

// fn find_frontmatter(content: &String) -> String {}

fn convert_markdown_to_html(markdown_content: &String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = MarkdownParser::new_ext(markdown_content, options);

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
    file_name: String
}

impl BuiltContentFile {
    pub fn from_file_name(build_directory: &Path, file_name: &String) -> Self {
        let mut path = std::path::PathBuf::new();

        let formatted_path = String::from(format!(
            "{}/{}.html",
            build_directory
                .as_os_str()
                .to_str()
                .expect("Could not convert articles build directory in new file path"),
            file_name 
        ));

        path.push(std::path::Path::new(&formatted_path));

        Self {
            path,
            file_name: formatted_path
        }

    }
}
