use serde::{Serialize, Deserialize};
use serde_json::{Result};
use html2md::parse_html;
use termimad::inline;
use owo_colors::OwoColorize;
use chrono::prelude::*;

static URL: &str = "https://2ch.hk/b/threads.json";

#[derive(Serialize, Deserialize)]
struct Thread {
    comment: String,
    lasthit: u32,
    num: String,
    posts_count: u16,
    score: f32,
    subject: String,
    timestamp: i64,
    views: u32
}

#[derive(Serialize, Deserialize)]
struct Threadmin {
    board: String,
    threads: Vec<Thread>
}

fn main() -> Result<()> {
    let response = reqwest::blocking::get(URL).unwrap();
    let desir: Threadmin = serde_json::from_str(&response.text().unwrap())?;
    let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(desir.threads[0].timestamp, 0), Utc);
    let timestampfmt = timestamp.format("%Y-%m-%d %H:%M:%S");

    println!("{}: {}\n{}: {}\n{}: {}\n{}: {}\n {}\n",
        "Board:".bold(), desir.board,
        "Thread".bold(), desir.threads[0].num,
        "Posts".bold(), desir.threads[0].posts_count,
        "Time".bold(), timestampfmt,
        inline(&parse_html(&desir.threads[0].comment)));

    Ok(())
}
