use iv::boards::{dvach::Dvach, fourchan::Fourchan, ImageBoard};

static BOARD: &str = "b";

fn main() {
	println!("{:?}", Dvach.get_last_thread(BOARD.to_owned()));
	println!("{:?}", Fourchan.get_last_thread(BOARD.to_owned()));
	println!("{:?}", Dvach.get_threads(BOARD.to_owned()));
	println!("{:?}", Fourchan.get_threads(BOARD.to_owned()));
}
