use datetime::LocalDate;
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};
use std::error::Error;

// impl From<&str> for LocalDate {
//     fn from(str: &str) -> Self {

//     }
// }

struct Daily {
    date: String,
    titles: Vec<Title>,
}

struct Title {
    name: String,
}
fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://pocket.shonenmagazine.com/";
    let resp = reqwest::blocking::get(url)?;
    let document = Document::from_read(resp).unwrap();
    let daily_series = document.find(Class("daily-series-wrapper"));
    let dailies: Vec<Daily> = daily_series.filter_map(|n| convert_to_daily(n)).collect();

    Ok(())
}

fn convert_to_daily(node: Node) -> Option<Daily> {
    // println!("{:?}", node);
    let res = node.find(Class("date"));
    res.for_each(|x| {
        let txt = x.first_child().unwrap().as_text();
        println!("{}", txt.unwrap());
    });
    None
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
