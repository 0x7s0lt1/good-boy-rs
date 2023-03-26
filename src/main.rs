#![deny(warnings)]

use url::Url;
use good_boy::{
    crawler::{Crawler}
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



    let url = Url::parse(&args[0]).expect("Can't parse URL!");
    let crawler = Crawler::new(url.clone());

    match Crawler::get_site_map(&url).await {
        Ok(mut _sitemap) => {
            println!("Sitemap found!");
            crawler.crawl_from_sitemap(&mut _sitemap).await;
        }
        Err(_) => {
            println!("No sitemap found!");
            Crawler::crawl(&url.as_str());
        }
    };
}

