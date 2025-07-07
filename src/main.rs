use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use rss::Channel;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    parse_xml("https://news.ycombinator.com/rss").await?;
    // println!("{:?}", response);
    Ok(())
}

async fn fetch_rss(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?; 
    Ok(response)
}

async fn parse_xml(xml: &str) -> Result<Channel, Box<dyn Error>> {
    let content = fetch_rss(xml).await?;
    let channel = Channel::read_from(content.as_bytes())?;
    for item in &channel.items {    
        println!("{:?}", item.title.as_deref().unwrap_or(""));
        println!("{:?}", item.link.as_deref().unwrap_or(""));
    }
    Ok(channel)
}

