pub type ThreadID = String;
pub type Board = String;

pub struct Thread {
	pub id: ThreadID,
	pub board: Board
}

pub struct BoardRequest {
	pub board: Board,
	pub thread: ThreadID
}

#[derive(Clone)]
pub struct File {
	pub uri: String,
	pub thumbnail: String,
	pub name: String,
	pub name_original: Option<String>
}

// All comments should be in markdown format
pub struct Post {
	pub id: String,
	pub comment: String,
	pub timestamp: i64,
	pub email: String,
	pub files: Vec<File>,
	pub op: bool
}

pub struct ThreadInfo {
	pub thread: Thread,
	pub comment: String,
	pub posts_count: i32,
	pub timestamp: i64
}

pub struct CatalogThread {
	pub board: Board,
	pub board_name: String,
	pub comment: String,
	pub email: String,
	pub op: bool,
	pub posts_count: i32,
	pub files_count: i32,
	pub files: Vec<File>,
	pub timestamp: i64
}
