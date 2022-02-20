use iv::boards::{dvach, ImageBoard};

use termimad::inline;
use chrono::prelude::*;

static BOARD: &str = "b";

fn main() {
	let board = BOARD.to_owned();
	let thread = dvach::Dvach.get_last_thread(board.clone());

	let posts = dvach::Dvach.get_thread_posts(board, thread.id);

	for i in 0..posts.clone().into_iter().len() {
		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(posts[i].timestamp, 0), Utc);
		let post_output = if posts[i].email != "" {
			format!("**{} [{}]**\nEmail: {}\n{}\n", posts[i].id, timestamp.format("%Y-%m-%d %H:%M:%S"), posts[i].email, posts[i].comment)
		} else {
			format!("**{} [{}]**\n{}\n", posts[i].id, timestamp.format("%Y-%m-%d %H:%M:%S"), posts[i].comment)
		};

		let output = if posts[i].files.len() > 0 {
			format!("{}Files: {}\n", post_output, posts[i].files.len())
		} else {
			post_output
		};

		println!("{}", inline(&output));
	}
}
