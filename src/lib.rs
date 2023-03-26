pub mod crawler;

const XML_EXTENSION: &str = ".xml";
const SITEMAP_INDEX: &str = "sitemapindex";
const SITEMAP: &str = "sitemap";
const URLSET: &str = "urlset";
const LOC: &str = "loc";
// const EMAIL_REGEX: Regex = Regex::new(r"\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,}\\b");
// const VALID_EMAIL_REGEX: Regex = Regex::new(r"^\\w+([\\.-]?\\w+)*@\\w+([\\.-]?\\w+)*(\\.\\w{2,3})+$");

const MINUS_ONE: i32 = -1;
const DISALLOWED_EXTENSIONS: [&str; 29] = [
    ".jpg", ".jpeg", ".bmp", ".tiff", ".gif", ".avif", ".apng", ".svg", ".ico", ".png", ".webp",
    ".mp3", ".wav", ".ogg", ".mp4", ".mov", ".wmv", ".avi", ".webm", ".flv", ".mkv", ".mts",
    ".m3u", ".ttf", ".otf", ".woff", ".woff2", ".eot", ".css",
];
