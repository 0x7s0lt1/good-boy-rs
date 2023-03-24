#![deny(warnings)]

use good_boy::sitemap::{crawl_from_sitemap, get_site_map};
use url::Url;

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

    match get_site_map(url).await {
        Ok(mut _sitemap) => {
            println!("Sitemap found!");
            crawl_from_sitemap(&mut _sitemap).await;
        }
        Err(_) => {
            println!("No sitemap found!");
        }
    };
}
