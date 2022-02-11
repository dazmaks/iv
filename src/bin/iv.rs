use iv::boards::{dvach, ImageBoard};

use termimad::inline;
use chrono::prelude::*;

static BOARD: &str = "b";

fn main() {
    let thread = dvach::Dvach.get_last(BOARD.to_owned());
    let timestamp: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(thread.timestamp, 0), Utc);
    let timestampfmt = timestamp.format("%Y-%m-%d %H:%M:%S");

    let output = format!(
"- **Board**: {}
- **Thread**: {}
- **Posts**: {}
- **Time**: {}

{}", thread.board, thread.id, thread.posts_count, timestampfmt, thread.comment);
    println!("{}", inline(&output));

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
