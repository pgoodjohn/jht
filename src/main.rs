use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    let all_content =
        std::fs::read_dir(std::path::Path::new("./content")).expect("could not find content dir");

    println!("{:?}", all_content);

    let mut all_articles = Vec::new();

    for entry in all_content.into_iter() {
        println!("{:?}", entry);

        let unwrapped = entry.unwrap();

        let file_contents = std::fs::read_to_string(unwrapped.path()).expect("unable to read file");

        let article_template =
            std::fs::read_to_string(std::path::Path::new("./templates/article.html"))
                .expect("article template missing");

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

        let mut new_file =
            File::create(std::path::Path::new(&new_file_name)).expect("failed to create a file");

        new_file
            .write_all(article_page.as_bytes())
            .expect("unable to write to article page");

        all_articles.push(new_file_name)
    }

    let mut article_hrefs = String::new();

    for article in all_articles {
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

    let index_template = std::fs::read_to_string(std::path::Path::new("./templates/index.html"))
        .expect("home templates missing");

    let mut new_index_page = File::create(std::path::Path::new("./build/index.html"))
        .expect("unable to create new listing page");

    new_index_page
        .write_all(index_template.as_bytes())
        .expect("could not create new index");
}
