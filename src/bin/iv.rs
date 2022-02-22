use iv::boards::{dvach::Dvach, ImageBoard};
use iv::types::Thread;

//use termimad::inline;
//use chrono::prelude::*;
use std::fs;
use std::io;

static BOARD: &str = "b";
static DIRECTORY: &str = "content";

fn main() {
	let threads = Dvach.get_threads(BOARD.to_owned());
	fs::create_dir(DIRECTORY).unwrap();
	for j in 0..threads.len() {
		let posts = Dvach.get_thread_posts(Thread{board: BOARD.to_owned(), id: threads[j].thread.id.clone()});

		// Downloading all files from board
		for i in 0..posts.len() {
			for file in posts[i].files.clone() {
				println!("Downloading: {}", file.name);
				let mut resp = reqwest::blocking::get(format!("https://2ch.hk{}", file.uri)).expect("request failed");
				let mut out = fs::File::create(format!("{}/{}", DIRECTORY, file.name)).expect("failed to create file");
				io::copy(&mut resp, &mut out).expect("failed to copy content");
			}

			/*
			let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(posts[i].timestamp, 0), Utc);
			let post_output = if posts[i].email != "" {
				format!("**{} [{}]**\nEmail: {}\n{}\n", posts[i].id, timestamp.format("%Y-%m-%d %H:%M:%S"), posts[i].email, posts[i].comment)
			} else {
				format!("**{} [{}]**\n{}\n", posts[i].id, timestamp.format("%Y-%m-%d %H:%M:%S"), posts[i].comment)
			};

			let output = if posts[i].files.len() > 0 {
				format!("{}Files: [{}]\n", post_output, posts[i].files.len())
			} else {
				post_output
			};

			println!("{}", inline(&output));
			*/
		}
	}
}
