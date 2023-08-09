use crawler::crawl_sites;
use text_processing::stop_word_removal;

mod crawler;
mod text_processing;

#[tokio::main]
async fn main() {
    // println!("{}", stop_word_removal(String::from("gand and big gaand")));
    crawl_sites(
        String::from("https://en.wikipedia.org/wiki/Kingdom_Two_Crowns"),
        5,
    )
    .await;
}
