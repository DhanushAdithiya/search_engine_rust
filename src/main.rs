use crawler::crawl_sites;

mod crawler;
mod process_query;
mod text_processing;

#[tokio::main]
async fn main() {
    let data = crawl_sites(String::from("https://en.wikipedia.org/wiki/Elephant"), 100).await;

    let prod_data = text_processing::tf_idf(data);

    let query = String::from("Africa");
    let pros_query = text_processing::query_if(query);

    println!(
        "SITE: {}",
        process_query::process_query(pros_query, prod_data)
    );
}
