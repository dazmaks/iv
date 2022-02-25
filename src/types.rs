pub type ThreadID = String;
pub type Board = String;

#[derive(Debug)]
pub struct Thread {
	pub id: ThreadID,
	pub board: Board
}

#[derive(Debug)]
pub struct BoardRequest {
	pub board: Board,
	pub thread: ThreadID
}

#[derive(Clone, Debug)]
pub struct File {
	pub uri: String,
	pub thumbnail: String,
	pub name: String,
	pub name_original: Option<String>
}

// All comments should be in markdown format
#[derive(Debug)]
pub struct Post {
	pub id: String,
	pub comment: String,
	pub timestamp: i64,
	pub email: String,
	pub files: Vec<File>,
	pub op: bool
}

#[derive(Debug)]
pub struct ThreadInfo {
	pub thread: Thread,
	pub comment: String,
	pub posts_count: i32,
	pub timestamp: i64
}

#[derive(Debug)]
pub struct Catalog {
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
