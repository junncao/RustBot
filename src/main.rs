use reqwest;
use rss::Channel;
use serde_json::json;
use tokio::time::{Instant, Duration, sleep_until};
use chrono::{Local, Timelike};

async fn fetch_and_send_news() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://rustcc.cn/rss";
    let body = reqwest::get(url).await?.text().await?;

    let channel = body.parse::<Channel>()?;

    let filter_keywords = vec!["招聘", "招人"]; // Add more keywords to this vector as needed

    let mut count = 0;
    let mut news = String::from("今日Rust社区热帖\n");
    'outer: for item in channel.items() {
        let title = item.title().unwrap_or("No title");
        for keyword in &filter_keywords {
            if title.contains(keyword) {
                continue 'outer;
            }
        }
        news.push_str(&format!("{}\n", title));
        news.push_str(&format!("{}\n\n", item.link().unwrap_or("No link")));
        count += 1;
        if count >= 5 {
            break;
        }
    }

    let client = reqwest::Client::new();
    let res = client.post("https://kim-robot.kwaitalk.com/api/robot/send?key=03bc4db2-66ee-4da4-b0e7-e5b005d9938f")
        .header("Content-Type", "application/json")
        .body(json!({
            "msgtype": "text",
            "text": {
                "content": news
            }
        }).to_string())
        .send()
        .await?;

    println!("Response: {:?}", res);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch_and_send_news().await?;

    loop {
        let now = Local::now();
        let next_ten_am = if now.hour() < 10 {
            now.date().and_hms(10, 0, 0)
        } else {
            now.date().succ().and_hms(10, 0, 0)
        };
        let until_next_ten_am = (next_ten_am - now).to_std().unwrap_or_else(|_| Duration::from_secs(0));
        sleep_until(Instant::now() + until_next_ten_am).await;

        fetch_and_send_news().await?;
    }
}
