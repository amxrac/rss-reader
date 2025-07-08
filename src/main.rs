use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use clap::builder::Str;
use rss::Channel;
use sqlx::sqlite::SqlitePool;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_db().await?;
    // add_feed
    // validate_rss
    parse_xml("https://news.ycombinator.com/rss").await?;
    Ok(())
}

async fn fetch_rss(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?; 
    Ok(response)
}

async fn parse_xml(xml: &str) -> Result<Channel, Box<dyn Error>> {
    let content: String = fetch_rss(xml).await?;
    let channel: Channel = Channel::read_from(content.as_bytes())?;
    // for item in &channel.items {    
    //     println!("{}", item.title.as_deref().unwrap_or(""));
    //     println!("{}", item.link.as_deref().unwrap_or(""));
    //     println!("{}", item.pub_date.as_deref().unwrap_or(""));
    // }
    Ok(channel)
}

async fn init_db() -> Result<(), Box<dyn Error>> {
    let pool = SqlitePool::connect("sqlite:feeds.db?mode=rwc").await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}

async fn validate_rss(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await.map_err(|_|"failed to fetch url")?; 
    let content = response.text().await.map_err(|_| "failed to read content")?;
    let channel = Channel::read_from(content.as_bytes()).map_err(|_| "invalid rss format")?;
    Ok(channel.title)
}
