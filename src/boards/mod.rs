pub mod dvach;

use crate::types::{Board, ThreadInfo};

pub trait ImageBoard {
	fn get_last(&self, board: Board) -> ThreadInfo;
	//fn get_available(&self, board: Board) -> Vec<ThreadInfo>;
	//fn get_thread(&self, req: BoardRequest) -> Thread;
}
