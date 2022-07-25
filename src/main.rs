use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Next steps:
/// - Init Script (creates index.html, listing.html, config.toml)
/// - Config reading
/// - Styles
///

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long, global = true)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialise Configuration
    Init,
    /// Build your website
    Build,
    /// Manage your configuration
    Config,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            init_command();
        }
        Some(Commands::Build) => {
            build_command();
        }
        Some(Commands::Config) => {
            config_command();
        }
        None => {
            failure_message();
        }
    }

    return;
}

fn build_command() {
    println!("Build command");
    // Get a config object as parameter

    // Build index.html
    build_index().expect("Could not build index.html");

    // Build content pages
    let content_list = build_content_pages(std::path::Path::new("./templates/article.html"))
        .expect("Could not build content");

    // Build content list page
    build_listing_page(content_list).expect("Could not build listing page");

    // Build other pages
}

fn init_command() {
    println!("Init command");
}

fn config_command() {
    println!("Config command");
}

fn failure_message() {
    println!("No command given");
}

fn build_index() -> Result<(), ()> {
    println!("Creating index.html from templates.index.html");
    let index_template = std::fs::read_to_string(std::path::Path::new("./templates/index.html"))
        .expect("home templates missing");

    let mut new_index_page = File::create(std::path::Path::new("./build/index.html"))
        .expect("unable to create new listing page");

    new_index_page
        .write_all(index_template.as_bytes())
        .expect("could not create new index");
    Ok(())
}

struct ContentList {
    items: Vec<String>,
}

fn build_content_pages(content_page_template: &Path) -> Result<ContentList, ()> {
    println!(
        "Building content pages with template {}",
        content_page_template.as_os_str().to_str().unwrap()
    );

    let all_content =
        std::fs::read_dir(std::path::Path::new("./content")).expect("could not find content dir");

    // todo!("Create a folder for the content it it doesn't already exist or it will crash");

    let mut all_articles = Vec::new();

    for entry in all_content.into_iter() {
        let unwrapped = entry.unwrap();

        let file_contents = std::fs::read_to_string(unwrapped.path()).expect("unable to read file");

        let article_template =
            std::fs::read_to_string(content_page_template).expect("article template missing");

        let article_page = article_template.replace("{article}", &file_contents);

        let new_file_name = String::from(format!(
            "./build/articles/{}.html",
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

fn build_listing_page(content_list: ContentList) -> Result<(), ()> {
    println!("Adding {:?} to listing page", content_list.items);

    let mut article_hrefs = String::new();

    for article in content_list.items {
        article_hrefs.push_str(&format!(
            "<a href={}>{}</a> <br />",
            article.strip_prefix("./build/").expect("big bad"),
            article.strip_prefix("./build/").expect("big bad")
        ));
    }

    let list_template = std::fs::read_to_string(std::path::Path::new("./templates/listing.html"))
        .expect("listing templates missing");

    let list_page = list_template.replace("{article_list}", &article_hrefs);

    let mut new_article_list = File::create(std::path::Path::new("./build/listing.html"))
        .expect("unable to create new listing page");

    new_article_list
        .write_all(list_page.as_bytes())
        .expect("could not create new listing file");

    Ok(())
}
