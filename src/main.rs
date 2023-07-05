use reqwest;
use rss::Channel;
use serde_json::json;
use tokio::time::{Instant, Duration, sleep_until};
use chrono::{Local, Timelike};

async fn send_bilibili(index: i32) -> Result<(), Box<dyn std::error::Error>> {
    for i in index..(index+3)  {
        let client = reqwest::Client::new();
        let res = client.post("https://kim-robot.kwaitalk.com/api/robot/send?key=a1f19682-5857-4a01-b1eb-33409d327e62")
            .header("Content-Type", "application/json")
            .body(json!({
                "msgtype": "text",
                "text": {
                    "content": format!("https://www.bilibili.com/video/BV1hp4y1k7SV?p={}", i)
                }
            }).to_string())
            .send()
            .await?;
        println!("Response: {:?}", res);
        
    }

    Ok(())


}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut index: i32 = 15;
    send_bilibili(index).await?;
    index += 3;

    loop {
        let now = Local::now();
        let next_ten_am = if now.hour() < 13 {
            now.date().and_hms(12, 30, 0)
        } else {
            now.date().succ().and_hms(12, 30, 0)
        };
        let until_next_ten_am = (next_ten_am - now).to_std().unwrap_or_else(|_| Duration::from_secs(0));
        sleep_until(Instant::now() + until_next_ten_am).await;

        send_bilibili(index).await?;
        index += 3;
    }
}
