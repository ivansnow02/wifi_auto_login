use reqwest;
use reqwest::Client;
use std::env;
use std::io::Write;
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .expect("Failed to open log file");
    let timestamp = chrono::Local::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        writeln!(file, "Argument missing at {}", timestamp).expect("Failed to write to log file");
        return;
    }
    let url = &args[1];
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to build client");

    let response = client.get(url).send().await;

    match response {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => {
                writeln!(file, "Request successful at {}", timestamp)
                    .expect("Failed to write to log file");
            }
            _ => {
                writeln!(
                    file,
                    "Request failed with status: {} at {}",
                    response.status(),
                    timestamp
                )
                .expect("Failed to write to log file");
            }
        },
        Err(e) => {
            writeln!(file, "Request error: {} at {}", e, timestamp)
                .expect("Failed to write to log file");
        }
    }
}
