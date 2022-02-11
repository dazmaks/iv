pub mod dvach;

use crate::types::{Board, ThreadInfo, CatalogThread};

pub trait ImageBoard {
	fn get_last(&self, board: Board) -> ThreadInfo;
	fn get_catalog(&self, board: Board) -> Vec<CatalogThread>;
	//fn get_available(&self, board: Board) -> Vec<ThreadInfo>;
	//fn get_thread(&self, req: BoardRequest) -> Thread;
}
