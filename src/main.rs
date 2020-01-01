extern crate regex;

use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut stack: Vec<String> = vec![];

    let res = reqwest::get("http://aoj-icpc.ichyo.jp/").await?;

    println!("{{\n\"AOJ-ICPCProblemIds\":");

    let html = res.text().await?;
    let document = Html::parse_document(&html);
    let css = "tr td a";
    let selector = Selector::parse(css).unwrap();

    let re = Regex::new(r"id=(\d{4})").unwrap();

    for node in document.select(&selector) {
        if node.value().attr("target").unwrap_or("") == "_blank" {
            let test = node.value().attr("href").unwrap_or("").to_string();
            let caps = re.captures(&test);
            match caps {
                Some(ret) => {
                    stack.push((&ret[1]).to_string());
                }
                None => {}
            }
        }
    }

    let uniq = stack
        .into_iter()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();
    println!("{:?}", uniq);

    println!("}}");

    Ok(())
}
