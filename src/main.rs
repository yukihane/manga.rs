use datetime::LocalDate;
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
    url: Url,
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
    let daily_series = document.find(Class("daily-series-wrapper"));
    let dailies: Vec<Daily> = daily_series.map(|n| convert_to_daily(n)).collect();

    println!("{:?}", dailies);
    Ok(())
}

fn convert_to_daily(node: Node) -> Daily {
    let date = node.find(Class("date")).map(|n| n.text()).next().unwrap();
    let titles: Vec<String> = node
        .find(Class("daily-series-title"))
        .map(|n| n.text())
        .collect();
    Daily { date, titles }
}

fn hacker_news(url: &str) {
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("div"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}
