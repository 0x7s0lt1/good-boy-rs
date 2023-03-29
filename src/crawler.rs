use crate::{DISALLOWED_EXTENSIONS, LOC, MINUS_ONE, SITEMAP, SITEMAP_INDEX, URLSET, XML_EXTENSION};
use minidom::Element;
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue};
use std::fs::OpenOptions;
use std::io::prelude::*;
use url::Url;

pub struct Crawler {
    pub url: Url,
    seen: Vec<String>,
    found: Vec<String>,
    url_regex: Regex,
    headers: HeaderMap,
    email_regex: Regex,
}

impl Crawler {
    pub fn new(url: Url) -> Self {

        let url_regex_string = format!(
            "{}|{}",
            url.host_str().unwrap(),
            url.as_str().replace("www.", "")
        );
        let url_regex = Regex::new(url_regex_string.as_str()).unwrap();

        Self {
            url,
            url_regex,
            ..Default::default()
        }
    }

    pub async fn crawl(&mut self, url: Option<&str>) {
        match self.format_url(url.unwrap_or(self.url.as_str() ) ) {
            Ok(url) => {
                if !&self.seen.contains(&url) {
                    self.seen.push(url.to_string());

                    let _res = reqwest::get(&url.to_string()).await.unwrap();
                    let _body = _res.text().await.unwrap();

                    self.email_regex
                        .find_iter(&_body)
                        .map(|e| e.as_str())
                        .for_each(|email| {
                            if !self.found.contains(&email.to_string()) {
                                self.found.push(email.to_string());

                                let mut file =
                                    OpenOptions::new().append(true).open("emails.txt").unwrap();

                                writeln!(file, "{}\r\n", email).expect("TODO: panic message");

                            }
                        });

                    println!("{}", url);
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }

    fn format_url(&self, url: &str) -> Result<String, &str> {
        if DISALLOWED_EXTENSIONS.iter().any(|ext| url.ends_with(ext)) {
            return Err("Extension not supported!");
        }

        if url.starts_with("http://") || url.starts_with("https://") {
            if self.url_regex.find_iter(url).count() as i32 > MINUS_ONE {
                return Ok(url.to_string());
            }
            return Err("URL is pointing to far!");
        }

        if url.starts_with("./") {
            return Ok(url.replace("./", ""));
        }
        if url.starts_with("../") {
            return Ok(url.replace("../", ""));
        }
        if !url.starts_with('/') {
            return Ok(format!("{}/{}", self.url, url));
        }

        Ok(format!("{}{}", self.url, url))
    }

    pub async fn get_site_map(&self) -> Result<Vec<String>, ()> {
        let robots_txt_uri = [self.url.as_str(), "robots.txt"].join("");

        let res = reqwest::get(robots_txt_uri).await.unwrap();
        let body = res.text().await.unwrap();

        let parts: Vec<String> = body
            .split('\n')
            .map(|s| s.to_string())
            .filter(|part| part.trim().starts_with("Sitemap") || part.trim().starts_with("sitemap"))
            .collect();

        if parts.len() > 0 { Ok(parts) } else { Err(()) }

    }

    pub async fn crawl_from_sitemap(&mut self, sitemaps: Vec<String>) {
        let _regex = Regex::new(r"(?i)sitemap:").unwrap();

        println!("{:?}", sitemaps);
        for map in sitemaps {
            let _replaced = map.replace('\r', "");
            let _split: Vec<_> = _regex.split(_replaced.as_str()).collect();

            let _range = _split[1].find(XML_EXTENSION).unwrap() + XML_EXTENSION.len();
            let _sitemap_uri = &_split[1][0.._range].trim().to_string();

            self.handle_sitemap_entry(_sitemap_uri).await.unwrap();
        }
    }

    pub async fn handle_sitemap_entry(
        &mut self,
        _sitemap_uri: &String,
    ) -> Result<(), reqwest::Error> {

        match Self::get_sitemap_xml(_sitemap_uri).await {
            Ok(root) => {
                if root.name() == SITEMAP_INDEX {
                    self.handle_sitemap_index(&root).await.unwrap();
                } else if root.name() == URLSET {
                    self.handle_sitemap(&root).await;
                }
            }
            Err(_) => {
                self.crawl(None).await;
            }
        }

        Ok(())
    }

    pub async fn handle_sitemap_index(&mut self, root: &Element) -> Result<(), reqwest::Error> {
        for child in root.children() {
            if child.name() == SITEMAP {
                for loc in child.children() {
                    if loc.name() == LOC {
                        let _root = Self::get_sitemap_xml(&loc.text()).await.unwrap();
                        self.handle_sitemap(&_root).await;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn handle_sitemap(&mut self, root: &Element) {
        for child in root.children() {
            for loc in child.children() {
                if loc.name() == LOC {
                    self.crawl(Some(&loc.text()) ).await;
                }
            }
        }
    }

    pub async fn get_sitemap_xml(_sitemap_uri: &String) -> Result<Element, reqwest::Error> {
        let resp = reqwest::get(_sitemap_uri).await?;
        let xml = resp.text().await?;

        let root: Element = xml.parse().unwrap();

        Ok(root)
    }

}

impl Default for Crawler {
    fn default() -> Self {

        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static("Good Boy"));

        Self {
            url: Url::parse("https://github.com/zsoltfehervari/good-boy-rs").unwrap(),
            headers,
            seen : Vec::new(),
            found: Vec::new(),
            url_regex: Regex::new("").unwrap(),
            email_regex: Regex::new(r"\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}\\b")
                .unwrap(),
        }
    }
}
