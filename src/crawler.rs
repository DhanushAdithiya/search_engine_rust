use reqwest;
use scraper::{self, html, Html, Selector};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use url;

use crate::text_processing::{self, stop_word_removal};

//TODO
// add a robots.txt implementaion
// implement a delay
// storing and indexing

pub async fn crawl_sites(starting_url: String, max_pages: usize) {
    let mut queue: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new(); //Hash prevents duplicates

    queue.push(starting_url);

    while !queue.is_empty() && visited.len() < max_pages {
        let current_url = queue.remove(0);

        if visited.contains(&current_url) {
            continue;
        }

        visited.insert(current_url.clone());
        if let Ok((page_content, clean)) = fetch_page_content(&current_url).await {
            if let Ok(urls) = fetch_url(page_content, max_pages - visited.len()).await {
                for url in urls {
                    if !visited.contains(&url) {
                        queue.push(url);
                    }
                }
            } else {
                eprintln!("Error while fetching URLs from page: {}", current_url);
            }
        } else {
            eprintln!("THERE WAS AN ERROR CRAWLLING ")
        }

        println!(
            "Queue length: {}, Visited pages: {}",
            queue.len(),
            visited.len()
        );
    }

    println!("{:?}", visited);
}

async fn fetch_page_content(url: &String) -> Result<(String, String), reqwest::Error> {
    thread::sleep(Duration::from_secs(1)); //get this value from robots.txt
    let content = reqwest::get(url).await?.text().await?;
    let html: Html = Html::parse_document(&content);
    let selectors = Selector::parse("h1,h2,h3,h4,h5,h6,a,p").unwrap();
    let selected_text = html
        .select(&selectors)
        .map(|element| element.text().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");
    let clean_puncuations = selected_text.replace(
        &['(', ')', ',', '"', '\"', '^', '.', ';', ':', '\''][..],
        "",
    );
    let clean_text = stop_word_removal(clean_puncuations);
    println!("{:?}", clean_text);
    Ok((content, clean_text))
}

async fn fetch_url(content: String, max: usize) -> Result<Vec<String>, reqwest::Error> {
    let parsed = scraper::Html::parse_fragment(&content);
    let anchor = scraper::Selector::parse("a").unwrap();
    let mut urls: Vec<String> = Vec::new();
    for element in parsed.select(&anchor) {
        let href_attr = element.value().attr("href").unwrap_or("");
        if !href_attr.is_empty() {
            //Filtering and Normalization

            if let Ok(parsed) = url::Url::parse(href_attr) {
                let host = parsed.domain().unwrap_or_default();
                //add the https://{url}/ tag here
                let link: String = format!("https://{}/", host);
                urls.push(link)
            };

            if urls.len() >= max {
                break;
            }
        }
    }
    Ok(urls)
}
