use crate::XML_EXTENSION;
// use quick_xml::events::Event;
// use quick_xml::reader::Reader;
use crate::util::parse_xml_into_vector;
use regex::Regex;
use url::Url;

pub async fn get_site_map(url: Url) -> Result<Vec<String>, reqwest::Error> {
    let robots_txt_uri = [url.as_str(), "robots.txt"].join("");

    let res = reqwest::get(robots_txt_uri).await?;
    let body = res.text().await?;

    let parts: Vec<String> = body
        .split('\n')
        .map(|s| s.to_string())
        .filter(|part| part.trim().starts_with("Sitemap") || part.trim().starts_with("sitemap"))
        .collect();

    Ok(parts)
}

pub async fn crawl_from_sitemap(sitemaps: &mut Vec<String>) {
    let _regex = Regex::new(r"(?i)sitemap:").unwrap();

    for map in sitemaps {
        let _replaced = map.replace('\r', "");
        let _split: Vec<_> = _regex.split(_replaced.as_str()).collect();

        let _range = &_split[1].find(XML_EXTENSION).unwrap() + XML_EXTENSION.len();
        let _sitemap_uri = &_split[1][0.._range].trim();

        handle_sitemap(_sitemap_uri).await.unwrap();
    }
}

pub async fn handle_sitemap(_sitemap_uri: &str) -> Result<String, reqwest::Error> {

    let resp = reqwest::get(_sitemap_uri.to_string()).await?;
    let xml = resp.text().await?;

    let vector = parse_xml_into_vector(xml.as_str(), "sitemapindex");
    println!("{:?}", vector);

    // let mut reader = Reader::from_str(xml.as_str());
    // reader.trim_text(true);
    //
    // let mut count = 0;
    // let mut txt = Vec::new();
    // let mut buf = Vec::new();
    //
    // loop {
    //     match reader.read_event_into(&mut buf) {
    //         Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
    //         Ok(Event::Eof) => break,
    //         Ok(Event::Start(e)) => match e.name().as_ref() {
    //             b"sitemapindex" => println!(
    //                 "attributes values: {:?}",
    //                 e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
    //             ),
    //             b"urlset" => count += 1,
    //             _ => (),
    //         },
    //         Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
    //
    //         // There are several other `Event`s we do not consider here
    //         _ => (),
    //     }
    //     // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
    //     buf.clear();
    // }

    Ok("ok".to_string())
}

// pub async fn crawl(path: String) -> usize {
//     0
// }
