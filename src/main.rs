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

#[derive(Debug)]
struct Daily {
    date: String,
    titles: Vec<String>,
}

#[derive(Debug)]
struct Work {
    id: String,
    name: String,
    url: String,
    episodes: Vec<Episode>,
}

#[derive(Debug)]
struct Episode {
    name: String,
    url: Url,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://pocket.shonenmagazine.com/";
    let resp = reqwest::blocking::get(url)?;
    let document = Document::from_read(resp).unwrap();
    let daily_series_items = document.find(Class("daily-series-item"));
    let works: Vec<Work> = daily_series_items.map(|n| convert_to_work(n)).collect();

    // println!("{:?}", works);
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
    println!("title: {}", title);
    println!("==========");
    // println!("{:?}", feed);

    Work {
        id,
        name: "".to_string(),
        url: "".to_string(),
        episodes: vec![],
    }
}

fn convert_to_daily(node: Node) -> Daily {
    let date = node.find(Class("date")).map(|n| n.text()).next().unwrap();
    let titles: Vec<String> = node
        .find(Class("daily-series-title"))
        .map(|n| n.text())
        .collect();
    Daily { date, titles }
}
