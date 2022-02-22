pub mod dvach;

use crate::types::{Board, ThreadInfo, CatalogThread, Thread, Post};

pub trait ImageBoard {
	fn get_last_thread(&self, board: Board) -> ThreadInfo;
	fn get_threads(&self, board: Board) -> Vec<ThreadInfo>;
	fn get_catalog(&self, board: Board) -> Vec<CatalogThread>;
	fn get_thread_posts(&self, thread: Thread) -> Vec<Post>;
}
