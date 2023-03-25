#![deny(warnings)]

use url::Url;
//use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use good_boy::{
    sitemap::{crawl_from_sitemap, get_site_map},
    crawler::{crawl}
};

#[tokio::main]
async fn main() {
    print!("\x1B[2J\x1B[1;1H");

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        println!("No Target-URL specified!");
        return;
    };
    if args[0] == "help" || args[0] == "h" {
        println!(
            "
  ▄████  ▒█████   ▒█████  ▓█████▄     ▄▄▄▄    ▒█████ ▓██   ██▓
 ██▒ ▀█▒▒██▒  ██▒▒██▒  ██▒▒██▀ ██▌   ▓█████▄ ▒██▒  ██▒▒██  ██▒
▒██░▄▄▄░▒██░  ██▒▒██░  ██▒░██   █▌   ▒██▒ ▄██▒██░  ██▒ ▒██ ██░
░▓█  ██▓▒██   ██░▒██   ██░░▓█▄   ▌   ▒██░█▀  ▒██   ██░ ░ ▐██▓░
░▒▓███▀▒░ ████▓▒░░ ████▓▒░░▒████▓    ░▓█  ▀█▓░ ████▓▒░ ░ ██▒▓░
 ░▒   ▒ ░ ▒░▒░▒░ ░ ▒░▒░▒░  ▒▒▓  ▒    ░▒▓███▀▒░ ▒░▒░▒░   ██▒▒▒
  ░   ░   ░ ▒ ▒░   ░ ▒ ▒░  ░ ▒  ▒    ▒░▒   ░   ░ ▒ ▒░ ▓██ ░▒░
░ ░   ░ ░ ░ ░ ▒  ░ ░ ░ ▒   ░ ░  ░     ░    ░ ░ ░ ░ ▒  ▒ ▒ ░░
      ░     ░ ░      ░ ░     ░        ░          ░ ░  ░ ░
                           ░               ░          ░ ░
        "
        );
        println!("  Usage: cargo run [url] [options]");
        println!("\r\n\r\n");
        return;
    }

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Good Boy"));

    let url = Url::parse(&args[0]).expect("Can't parse URL!");
    // let url_regex = Regex::new(&url.host().unwrap() + "|" + &url.as_str().replace("www.", "")).unwrap();
    //
    // let mut seen : Vec<String> = Vec::new();
    // let mut found: Vec<String> = Vec::new();

    match get_site_map(&url).await {
        Ok(mut _sitemap) => {
            println!("Sitemap found!");
            crawl_from_sitemap(&mut _sitemap).await;
        }
        Err(_) => {
            println!("No sitemap found!");
            crawl(&url.as_str()).await;
        }
    };
}

