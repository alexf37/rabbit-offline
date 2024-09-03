use std::env;
use std::process::exit;

use futures::future::join_all;
use regex::Regex;
use reqwest;
use std::error::Error;
use tokio;
use tokio::io::AsyncWriteExt;

async fn download_file(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    // dont write the file if it already exists
    if tokio::fs::metadata(file_path).await.is_ok() {
        println!("file already exists: {}", file_path);
        return Ok(());
    }

    let response = reqwest::get(url).await?;

    // ensure the directory exists
    if let Some(parent) = std::path::Path::new(file_path).parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    let mut file = tokio::fs::File::create(file_path).await?;
    let content = response.text().await?;

    // replace /wiki/{name} urls with ./wiki/{name}.html
    let re = Regex::new(r"/wiki/([a-zA-Z0-9%_-,\(\)]+)").unwrap();
    let result = re.replace_all(&content, "./wiki/$1.html");

    tokio::io::copy(&mut result.as_bytes().as_ref(), &mut file).await?;

    println!("downloaded {} to {}", url, file_path);

    Ok(())
}

async fn write_file(file_path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = tokio::fs::File::create(file_path).await?;
    file.write_all(content.as_bytes()).await?;
    return Ok(());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let wiki_url = match args.get(1) {
        Some(n) => n,
        None => {
            println!("Number argument missing");
            exit(1);
        }
    };
    let html = reqwest::get(wiki_url).await?.text().await?;

    let re = Regex::new(r#"href\s*=\s*["'](/wiki/[^"']*)["']"#).unwrap();
    let mut tasks = Vec::new();

    for cap in re.captures_iter(&html) {
        let url = format!("https://en.wikipedia.org{}", &cap[1]);
        let file_path = format!("./{}.html", &cap[1]);

        tasks.push(tokio::spawn(async move {
            if let Err(e) = download_file(&url, &file_path).await {
                eprintln!("error downloading {}: {}", url, e);
            }
        }));

        println!("href: {}", &cap[1]);
    }

    join_all(tasks).await;

    let re = Regex::new(r"/wiki/([a-zA-Z0-9_,\(\)-]+)").unwrap();
    let result = re.replace_all(&html, "./wiki/$1.html");

    write_file("./index.html", &result).await?;
    return Ok(());
}
