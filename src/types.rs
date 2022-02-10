pub type ThreadID = String;
pub type Board = String;

pub struct BoardRequest {
	pub board: Board,
	pub thread: ThreadID
}

pub struct File {
	pub uri: String,
	pub name: String,
	pub name_original: String
}

// All comments should be in markdown format

pub struct Post {
	pub comment: String,
	pub email: String,
	pub files: Option<Vec<File>>,
	pub op: bool
}

pub struct ThreadInfo {
	pub board: Board,
    pub id: ThreadID,
    pub comment: String,
	pub posts_count: i64,
	//pub files_count: i64,
	pub timestamp: i64
}

pub struct Thread {
	pub thread_info: ThreadInfo,
	pub posts: Vec<Post>
}
