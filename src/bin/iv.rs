extern crate iv;

use iv::boards::{dvach, ImageBoard};
use termimad::inline;
use owo_colors::OwoColorize;
use chrono::prelude::*;


static BOARD: &str = "b";

fn main() {
    let thread = dvach::Dvach.get_last(BOARD.to_owned());
    let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(thread.timestamp, 0), Utc);
    let timestampfmt = timestamp.format("%Y-%m-%d %H:%M:%S");

    println!("{}: {}\n{}: {}\n{}: {}\n{}: {}\n {}\n",
        "Board".bold(), thread.board,
        "Thread".bold(), thread.id,
        "Posts".bold(), thread.posts_count,
        "Time".bold(), timestampfmt,
        inline(&thread.comment));
}
