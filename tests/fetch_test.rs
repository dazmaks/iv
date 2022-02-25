use iv::boards::{dvach::Dvach, fourchan::Fourchan, ImageBoard};

static BOARD:&str = "b";

#[test]
fn get_url() {
	Dvach.get_url(BOARD.to_owned(), "foo-bar".to_owned());
	Fourchan.get_url(BOARD.to_owned(), "foo-bar".to_owned());
}

#[test]
fn get_last_thread() {
	Dvach.get_last_thread(BOARD.to_owned());
	Fourchan.get_last_thread(BOARD.to_owned());
}

#[test]
fn get_threads() {
	Dvach.get_threads(BOARD.to_owned());
	Fourchan.get_threads(BOARD.to_owned());
}

#[test]
fn get_catalog() {
	Dvach.get_catalog(BOARD.to_owned());
	Fourchan.get_catalog(BOARD.to_owned());
}

#[test]
fn get_thread_posts() {
	Dvach.get_thread_posts(
		Dvach.get_last_thread(BOARD.to_owned()).thread
	);
	Fourchan.get_thread_posts(
		Fourchan.get_last_thread(BOARD.to_owned()).thread
	);
}
