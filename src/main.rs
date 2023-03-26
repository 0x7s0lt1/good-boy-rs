#![deny(warnings)]

use dotenv::dotenv;
use good_boy::crawler::Crawler;
use url::Url;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").unwrap(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let _sentry = sentry::init(sentry::ClientOptions {
        release: sentry::release_name!(),
        traces_sample_rate: 0.2,
        ..Default::default()
    });

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

    let mut url_arg = args[0].clone();
    if !url_arg.ends_with('/') {
        url_arg = url_arg.to_owned() + "/";
    }

    let url = Url::parse(&url_arg).expect("Can't parse URL!");
    let mut crawler = Crawler::new(url.clone());

    match Crawler::get_site_map(&url).await {
        Ok(mut _sitemap) => {
            println!("Sitemap found!");
            crawler.crawl_from_sitemap(&mut _sitemap).await;
        }
        Err(_) => {
            println!("No sitemap found!");
            crawler.crawl(url.as_str()).await;
        }
    };
}
