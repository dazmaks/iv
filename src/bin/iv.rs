use iv::boards::{dvach, ImageBoard};

use termimad::inline;
use chrono::prelude::*;

static BOARD: &str = "b";

fn main() {
	let board = BOARD.to_owned();
	let thread = dvach::Dvach.get_last(board.clone());
	let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(thread.timestamp, 0), Utc);
	let timestampfmt = timestamp.format("%Y-%m-%d %H:%M:%S");

	let posts = dvach::Dvach.get_thread_posts(board, thread.id.clone());
	for i in 0..posts.clone().into_iter().len() {
		let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(posts[i].clone().timestamp, 0), Utc);
		let timestampfmt = timestamp.format("%Y-%m-%d %H:%M:%S");

		let poutput = format!("**[{}] - {}**\n{}\n", posts[i].id, timestampfmt, posts[i].comment);
		println!("{}", inline(&poutput));
	}

	/*
	let skin = MadSkin::default();

	ask!(&skin, "What's next?", ('e') {
		('s', "Save") => {
			mad_print_inline!(skin, "**Saving**\n");
			std::fs::write(format!("{}-{}.save.md", thread.board, thread.id), output.clone()).unwrap();
		}
		('e', "Exit iv") => {
			std::process::exit(0);
		}
	});
	*/
}
