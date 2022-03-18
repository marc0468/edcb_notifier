use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    discord: Detail,
    line: Detail,
    slack: Detail,
}
#[derive(Serialize, Deserialize, Debug)]
struct Detail {
    is_disabled: bool,
    #[serde(default)]
    webhook_url: String,
    #[serde(default)]
    token: String,
}

#[tokio::main]
async fn post_discord(url: &str, msg: &str) {
    let mut map = HashMap::new();
    map.insert("content", msg);
    let client = reqwest::Client::new();
    let _res = client.post(url).json(&map).send().await;
}
#[tokio::main]
async fn post_line(token: &str, msg: &str) {
    let url = "https://notify-api.line.me/api/notify";
    let client = reqwest::Client::new();
    let _res = client
        .post(url)
        .header("Authorization", format!("Bearer {token}", token = token))
        .header("content-type", "application/x-www-form-urlencoded")
        .body(format!("message={msg}", msg = msg))
        .send()
        .await;
}
#[tokio::main]
async fn post_slack(url: &str, msg: &str) {
    let mut map = HashMap::new();
    map.insert("text", msg);
    let client = reqwest::Client::new();
    let _res = client.post(url).json(&map).send().await;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let msg = &args[2];
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let deserialized: Config = serde_json::from_reader(reader).unwrap();

    if deserialized.discord.is_disabled {
        post_discord(&deserialized.discord.webhook_url, msg);
    }
    if deserialized.line.is_disabled {
        post_line(&deserialized.line.token, msg);
    }
    if deserialized.slack.is_disabled {
        post_slack(&deserialized.slack.webhook_url, msg);
    }
}
