use crate::types::{Board, ThreadInfo, CatalogThread, File, ThreadID, Post};

use html2md::parse_html;
use serde::de::{self, Deserialize, Deserializer, Unexpected};

#[allow(dead_code)]
#[derive(Clone)]
struct IntBool(bool);

impl<'de> Deserialize<'de> for IntBool {
    fn deserialize<D>(deserializer: D) -> Result<IntBool, D::Error>
	where
		D: Deserializer<'de>,
	{
		match u8::deserialize(deserializer)? {
			0 => Ok(IntBool(false)),
			1 => Ok(IntBool(true)),
			other => Err(de::Error::invalid_value(
				Unexpected::Unsigned(other as u64),
				&"zero or one",
			)),
		}
	}
}

// #[derive(Serialize, Deserialize)] D:

#[allow(dead_code)]
#[derive(Deserialize)]
struct ThreadShort {
	comment: String,
	lasthit: i32,
	num: String,
	posts_count: i32,
	score: f32,
	subject: String,
	timestamp: i64,
	views: i32
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ThreadsJSON {
	board: String,
	threads: Vec<ThreadShort>
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AbuNews {
	date: String,
	num: i32,
	subject: String,
	views: i32
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct DvachFile {
	displayname: String,
	fullname: Option<String>,
	height: i32,
	md5: Option<String>,
	name: String,
	nsfw: Option<u8>,
	path: String,
	size: i32,
	thumbnail: String,
	tn_height: i32,
	tn_width: i32,
	r#type: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CatalogRespThread {
	banned: IntBool,
	closed: IntBool,
	comment: String,
	date: String,
	email: String,
	endless: IntBool,
	files: Vec<DvachFile>,
	files_count: i32,
	lasthit: i32,
	name: String,
	num: String,
	op: IntBool,
	parent: String,
	posts_count: i32,
	sticky: IntBool,
	subject: String,
	tags: String,
	timestamp: i64,
	trip: String
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TopBoard {
	board: String,
	info: String,
	name: String
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CatalogResp {
	board: Board,
	board_info: String,
	board_info_outer: String,
	board_name: String,
	advert_bottom_image: String,
	advert_bottom_link: String,
	advert_mobile_image: String,
	advert_mobile_link: String,
	advert_top_image: String,
	advert_top_link: String,
	board_banner_image: String,
	board_banner_link: String,
	bump_limit: i32,
	default_name: String,
	enable_dices: IntBool,
	enable_flags: IntBool,
	enable_icons: IntBool,
	enable_images: IntBool,
	enable_likes: IntBool,
	enable_names: IntBool,
	enable_oekaki: IntBool,
	enable_posting: IntBool,
	enable_sage: IntBool,
	enable_shield: IntBool,
	enable_subject: IntBool,
	enable_thread_tags: IntBool,
	enable_trips: IntBool,
	enable_video: IntBool,
	filter: String,
	max_comment: i32,
	max_files_size: i32,
	news_abu: Vec<AbuNews>,
	threads: Vec<CatalogRespThread>,
	top: Vec<TopBoard>
}

#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct ThreadRespPost {
	banned: IntBool,
	closed: IntBool,
	comment: String,
	date: String,
	email: String,
	files: Vec<DvachFile>,
	lasthit: i32,
	name: String,
	num: i32,
	number: i32,
	op: IntBool,
	parent: String,
	sticky: IntBool,
	subject: String,
	timestamp: i64,
	trip: String
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ThreadRespThread {
	posts: Vec<ThreadRespPost>
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
struct ThreadResp {
	r#Board: Board,
	BoardInfo: String,
	BoardInfoOuter: String,
	BoardName: String,
	advert_bottom_image: String,
	advert_bottom_link: String,
	advert_mobile_image: String,
	advert_mobile_link: String,
	advert_top_image: String,
	advert_top_link: String,
	board_banner_image: String,
	board_banner_link: String,
	bump_limit: i32,
	current_thread: String,
	default_name: String,
	enable_dices: IntBool,
	enable_flags: IntBool,
	enable_icons: IntBool,
	enable_images: IntBool,
	enable_likes: IntBool,
	enable_names: IntBool,
	enable_oekaki: IntBool,
	enable_posting: IntBool,
	enable_sage: IntBool,
	enable_shield: IntBool,
	enable_subject: IntBool,
	enable_thread_tags: IntBool,
	enable_trips: IntBool,
	enable_video: IntBool,
	files_count: i32,
	is_board: IntBool,
	is_closed: IntBool,
	is_index: IntBool,
	max_comment: i32,
	max_files_size: i32,
	max_num: i32,
	news_abu: Vec<AbuNews>,
	posts_count: i32,
	thread_first_image: String,
	threads: Vec<ThreadRespThread>,
	title: String,
	top: Vec<TopBoard>
}

pub struct Dvach;

impl super::ImageBoard for Dvach {
	fn get_last_thread(&self, board: Board) -> ThreadInfo {
		let url: String = format!("https://2ch.hk/{}/threads.json", board);
		let response = reqwest::blocking::get(url).unwrap();
		let deserialized: ThreadsJSON = serde_json::from_str(&response.text().unwrap()).unwrap();
		ThreadInfo {
			board: deserialized.board,
			id: deserialized.threads[0].num.clone(),
			comment: parse_html(&deserialized.threads[0].comment),
			posts_count: deserialized.threads[0].posts_count,
			timestamp: deserialized.threads[0].timestamp
		}
	}

	fn get_threads(&self, board: Board) -> Vec<ThreadInfo> {
		let url: String = format!("https://2ch.hk/{}/threads.json", board);
		let response = reqwest::blocking::get(url).unwrap();
		let deserialized: ThreadsJSON = serde_json::from_str(&response.text().unwrap()).unwrap();
		let threads: Vec<ThreadInfo> = deserialized.threads.into_iter().map(|t|
			ThreadInfo { board: deserialized.board.clone(), id: t.num, comment: t.comment, posts_count: t.posts_count, timestamp: t.timestamp }).collect();
		threads
	}

	fn get_catalog(&self, board: Board) -> Vec<CatalogThread> {
		let url: String = format!("https://2ch.hk/{}/catalog.json", board);
		let response = reqwest::blocking::get(url).unwrap();
		let deserialized: CatalogResp = serde_json::from_str(&response.text().unwrap()).unwrap();
		let catalog: Vec<CatalogThread> = deserialized.threads.into_iter().map(|e|
			CatalogThread {
				board: deserialized.board.clone(),
				board_name: deserialized.board_name.clone(),
				comment: parse_html(&e.comment),
				email: e.email,
				op: e.op.0,
				posts_count: e.posts_count,
				files_count: e.files_count,
				files: e.files.into_iter().map(|f| File {
					uri: f.path,
					thumbnail: f.thumbnail,
					name: f.displayname,
					name_original: f.fullname
				}).collect(),
				timestamp: e.timestamp
			}).collect();
		catalog
	}

	fn get_thread_posts(&self, board: Board, thread: ThreadID) -> Vec<Post> {
		let url = format!("https://2ch.hk/{}/res/{}.json", board, thread);
		let response = reqwest::blocking::get(url).unwrap();

		let deserialized: ThreadResp = serde_json::from_str(&response.text().unwrap()).unwrap();
		let posts = deserialized.threads[0].posts.clone().into_iter().map(|p|
			Post {
				id: p.num.to_string(),
				comment: parse_html(&p.comment),
				timestamp: p.timestamp,
				email: p.email,
				files: p.files.into_iter().map(|f|
					File {
						uri: f.path,
						thumbnail: f.thumbnail,
						name: f.name,
						name_original: f.fullname
					}).collect(),
				op: p.op.0
			}).collect();
		posts
	}
}
