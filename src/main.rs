use std::error::Error;
use rss::Channel;
use sqlx::sqlite::SqlitePool;
use std::io;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_db().await?;
    menu().await?;
    Ok(())
}

async fn fetch_rss(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url).await?.text().await?; 
    Ok(response)
}

async fn parse_xml(xml: &str) -> Result<Channel, Box<dyn Error>> {
    let content: String = fetch_rss(xml).await?;
    let channel: Channel = Channel::read_from(content.as_bytes())?;
    for item in &channel.items {    
        println!("\n{}", item.title.as_deref().unwrap_or(""));
        println!("{}", item.link.as_deref().unwrap_or(""));
        println!("");
    }
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
                sqlx::query("INSERT INTO feeds (title, url, created_at) VALUES (?, ?, ?)")
                .bind(title)
                .bind(&url.trim())
                .bind(chrono::Utc::now().to_rfc3339())
                .execute(pool)
                .await?;
                println!("rss feed added successfully");
                break;
            }
            Err(err) => {
                println!("Error adding feed: {}", err);
                continue;
            }
        }

    };
    Ok(())
}

async fn display_items(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let urls: Vec<String> = sqlx::query_scalar("SELECT url FROM feeds")
    .fetch_all(pool)
    .await?;

    for url in &urls {
        parse_xml(url).await?;
    }
    Ok(())
}

async fn menu() -> Result<(), Box<dyn Error>>{
    let pool = SqlitePool::connect("sqlite:feeds.db").await?;

    loop {
        println!("\nenter 1 to add rss");
        println!("enter 2 to view feed");
        println!("enter 3 to exit\n");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        match input.trim() {
            "1" => add_feed(&pool).await?,
            "2" => display_items(&pool).await?,
            "3" => {
                println!("goodbye");
                break;
            }
            _ => println!("invalid option. enter either 1 to add rss, 2 to view feed, or 3 to exit"),
        }

    };

    Ok(())
}