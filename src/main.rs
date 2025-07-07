use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use rss::Channel;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let response = fetch_rss("https://news.ycombinator.com/rss").await?;
    println!("{:?}", response);
    Ok(())
}

async fn fetch_rss(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?; 
    Ok(response)
}