use reqwest;
use serde_json::json;
use tokio::time::{Instant, Duration, sleep_until};
use std::fs;
use chrono::{Local, Timelike};
use chrono::prelude::*;
use chatgpt::prelude::*;

async fn send_request() -> Result<()> {
    let now = Utc::now();
    println!("now date: {}", now);
    let key = "";

    let client = ChatGPT::new(key)?;
    let prompt = fs::read_to_string("src/prompt.txt")
        .expect("Failed to read file");

    let response = client
        .send_message(prompt)
        .await?;

    println!("Response: {}", response.message().content);

    
    let client = reqwest::Client::new();
    let res = client.post("https://kim-robot.kwaitalk.com/api/robot/send?key=92425d1b-d7a5-4340-9a7e-3633ac5eb638")
        .header("Content-Type", "application/json")
        .body(json!({
            "msgtype": "text",
            "text": {
                "content": response.message().content
            }
        }).to_string())
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())


}

#[tokio::main]
async fn main() -> Result<()> {
    let pid = std::process::id();
    println!("The current process ID is {}", pid);
    send_request().await?;
    loop {
        let now = Local::now();
        let next_ten_am = if now.hour() < 10 {
            now.date().and_hms(10, 0, 0)
        } else {
            now.date().succ().and_hms(10, 0, 0)
        };
        let until_next_ten_am = (next_ten_am - now).to_std().unwrap_or_else(|_| Duration::from_secs(0));
        sleep_until(Instant::now() + until_next_ten_am).await;
        send_request().await?;    
    }

    Ok(())
}
