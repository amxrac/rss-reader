use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use clap::builder::Str;
use clap::Parser;
use rss::Channel;
use sqlx::sqlite::SqlitePool;
use sqlx::Sqlite;
use std::io;
use chrono::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_db().await?;
    // parse_xml("https://news.ycombinator.com/rss").await?;
    let pool = SqlitePool::connect("sqlite:feeds.db").await?;
    add_feed(&pool).await?;
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

async fn validate_rss(url: &str) -> Result<String, String> {
    let response = reqwest::get(url).await.map_err(|_|"failed to fetch url")?; 
    let content = response.text().await.map_err(|_| "failed to read content")?;
    let channel = Channel::read_from(content.as_bytes()).map_err(|_| "invalid rss format")?;
    Ok(channel.title)
}

async fn add_feed(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    println!("enter rss feed url: ");
    let mut url: String = String::new();


    loop {
        url.clear();

        io::stdin()
            .read_line(&mut url)
            .expect("failed to read line");

        match validate_rss(&url).await {
            Ok(title) => {
                sqlx::query(
                    r#"
                    INSERT INTO feeds (title, url, published_at) VALUES (?, ?, ?)
                    "#
                )
                .bind(title)
                .bind(&url.trim())
                .bind(chrono::Utc::now().to_rfc3339())
                .execute(pool)
                .await?;
                println!("rss feed added successfully");
                break url.trim().to_string();
            }
            Err(err) => {
                println!("Error adding feed: {}", err);
                continue;
            }
        }

    };
    Ok(())
}