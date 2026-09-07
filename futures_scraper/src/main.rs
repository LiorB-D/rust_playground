use futures::{StreamExt, stream};
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

async fn fetch_hacker_news(client: &Client) -> Result<String, reqwest::Error> {
    client
        .get("https://news.ycombinator.com/")
        .send()
        .await?
        .text()
        .await
}

fn first_five_urls(html: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let selector = Selector::parse("tr.athing .titleline > a")?;

    Ok(Html::parse_document(html)
        .select(&selector)
        .take(5)
        .filter_map(|link| link.value().attr("href"))
        .map(str::to_owned)
        .collect())
}

async fn fetch_payload_size(
    client: &Client,
    url: String,
) -> Result<(String, usize), reqwest::Error> {
    let size = client.get(&url).send().await?.bytes().await?.len();
    Ok((url, size))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let urls = first_five_urls(&fetch_hacker_news(&client).await?)?;

    stream::iter(urls)
        .map(|url| fetch_payload_size(&client, url))
        .buffer_unordered(5)
        .for_each(|result| async {
            match result {
                Ok((url, size)) => println!("{url}: {size} bytes"),
                Err(error) => eprintln!("request failed: {error}"),
            }
        })
        .await;

    Ok(())
}
