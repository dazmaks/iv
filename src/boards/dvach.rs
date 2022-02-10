use crate::types::{Board, ThreadInfo};

use serde::{Serialize, Deserialize};
use html2md::parse_html;

#[derive(Serialize, Deserialize)]
struct Thread {
    comment: String,
    lasthit: u32,
    num: String,
    posts_count: i64,
    score: f32,
    subject: String,
    timestamp: i64,
    views: i64
}

#[derive(Serialize, Deserialize)]
struct ThreadArray {
    board: String,
    threads: Vec<Thread>
}

pub struct Dvach;

impl super::ImageBoard for Dvach {
	fn get_last(&self, board: Board) -> ThreadInfo {
		let url: String = format!("https://2ch.hk/{}/threads.json", board);
		let response = reqwest::blocking::get(url).unwrap();
		let desir: ThreadArray = serde_json::from_str(&response.text().unwrap()).unwrap();
		ThreadInfo {
            board: desir.board,
            id: desir.threads[0].num.clone(),
            comment: parse_html(&desir.threads[0].comment).clone(),
            posts_count: desir.threads[0].posts_count,
            timestamp: desir.threads[0].timestamp,
            views: desir.threads[0].views
        }
	}
}
