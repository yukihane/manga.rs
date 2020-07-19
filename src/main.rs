use datetime::LocalDate;
use feed_rs::parser;
use percent_encoding::*;
use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use std::error::Error;
use url::Url;

// impl From<&str> for LocalDate {
//     fn from(str: &str) -> Self {

//     }
// }

/// 作品
#[derive(Debug)]
struct Work {
    /// 作品ID
    id: String,
    /// 作品タイトル
    name: String,
    /// 作品URL
    url: String,
    /// エピソード
    episodes: Vec<Episode>,
}

/// エピソード
#[derive(Debug)]
struct Episode {
    /// エピソードタイトル
    name: String,
    /// エピソードリンク
    url: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://pocket.shonenmagazine.com/";
    let resp = reqwest::blocking::get(url)?;
    let document = Document::from_read(resp).unwrap();
    let daily_series_items = document.find(Class("daily-series-item"));
    let works: Vec<Work> = daily_series_items.map(|n| convert_to_work(n)).collect();

    Ok(())
}

fn convert_to_work(node: Node) -> Work {
    // https://cdn-img.pocket.shonenmagazine.com/public/series-thumbnail/10834108156725725519-4f953f8bbbd8164a9da0edda9fdbd6c3?1588140480
    let pattern =
        Regex::new(r"https://cdn-img.pocket.shonenmagazine.com/public/series-thumbnail/(\d+)-.*")
            .unwrap();
    let img = node
        .find(Name("img"))
        .next()
        .unwrap()
        .attr("data-src")
        .unwrap();
    let img = percent_encoding::percent_decode_str(img)
        .decode_utf8()
        .unwrap();

    let m = pattern.captures(&img).unwrap();
    let img_url = m.get(0).unwrap().as_str();
    // これが作品IDっぽい
    let id = m.get(1).unwrap().as_str().to_string();
    let rss_url = format!("https://pocket.shonenmagazine.com/rss/series/{}", id);

    let rss = reqwest::blocking::get(&rss_url).unwrap();
    let feed = parser::parse(Box::new(rss)).unwrap();
    let title = feed.title.unwrap().content.to_string();
    let link = feed.links.iter().next().unwrap().href.clone();
    let episodes: Vec<Episode> = feed
        .entries
        .iter()
        .map(|f| {
            let title = &f.title;
            let title = title.as_ref().unwrap().content.clone();
            let link = f.links.iter().next().unwrap().href.clone();
            Episode {
                name: title,
                url: link,
            }
        })
        .collect();

    Work {
        id,
        name: title,
        url: link,
        episodes,
    }
}
