use super::configuration;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser)]
pub struct BuildCommand {
    #[clap(short, long, global = true)]
    debug: bool,
}

pub fn command(_command: &BuildCommand, config: &configuration::Config) {
    println!("Build command");

    // Todo: initialize build directory if empty.

    // Build index.html
    build_index(&config.templates_dir, &config.build_config.build_directory)
        .expect("Could not build index.html");

    // Build content pages
    let content_list = build_content_pages(
        std::path::Path::new("./templates/article.html"),
        std::path::Path::new(&config.build_config.articles_directory),
        std::path::Path::new(&config.content_dir),
    )
    .expect("Could not build content");

    // Build content list page
    build_listing_page(
        content_list,
        std::path::Path::new(&config.templates_dir),
        std::path::Path::new(&config.build_config.build_directory),
        &config.build_config.article_listings_page,
    )
    .expect("Could not build listing page");

    // Build other pages
}

fn build_index(templates_directory: &String, build_directory: &String) -> Result<(), ()> {
    println!(
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
    articles_build_directory: &Path,
    content_directory: &Path,
) -> Result<ContentList, ()> {
    println!(
        "Building content pages with template {}",
        content_page_template.as_os_str().to_str().unwrap()
    );

    // todo!("Accept this as configuration parameter")
    let content_build_folder_path = std::path::Path::new(articles_build_directory);

    match content_build_folder_path.exists() {
        true => {}
        false => {
            println!(
                "Creating {}",
                content_build_folder_path
                    .to_str()
                    .expect("Could not unwrap string")
            );
            std::fs::create_dir(content_build_folder_path)
                .expect("Failed to create articles build folder in ./build/articles");
        }
    }

    let all_content = std::fs::read_dir(content_directory).expect("could not find content dir");

    // todo!("Create a folder for the content it it doesn't already exist or it will crash");

    let mut all_articles = Vec::new();

    for entry in all_content.into_iter() {
        let unwrapped = entry.unwrap();

        let file_contents = std::fs::read_to_string(unwrapped.path()).expect("unable to read file");

        let article_template =
            std::fs::read_to_string(content_page_template).expect("article template missing");

        let article_page = article_template.replace("{article}", &file_contents);

        let new_file_name = String::from(format!(
            "{}/{}.html",
            articles_build_directory
                .as_os_str()
                .to_str()
                .expect("Could not convert articles build directory in new file path"),
            unwrapped
                .path()
                .file_stem()
                .expect("big fail")
                .to_str()
                .expect("bigger fail")
        ));

        let mut new_file = File::create(std::path::Path::new(&new_file_name))
            .expect("failed to create a file to store the article");

        new_file
            .write_all(article_page.as_bytes())
            .expect("unable to write to article page");

        all_articles.push(new_file_name)
    }

    println!("Build content pages {:?}", all_articles);

    Ok(ContentList {
        items: all_articles,
    })
}

fn build_listing_page(
    content_list: ContentList,
    templates_directory: &Path,
    build_directory: &Path,
    article_listing_page_name: &String,
) -> Result<(), ()> {
    println!("Adding {:?} to listing page", content_list.items);

    let mut article_hrefs = String::new();

    for article in content_list.items {
        article_hrefs.push_str(&format!(
            "<a href={}>{}</a> <br />",
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
