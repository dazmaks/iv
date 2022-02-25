use std::vec;

use crate::types::{Board, ThreadInfo, Catalog, File, Post, Thread};
use crate::boards::IntBool;

use html2md::parse_html;

// https://github.com/4chan/4chan-API/blob/master/pages/Indexes.md
#[allow(dead_code)]
#[derive(Deserialize, Clone)]
struct FourPost {
    no: i32,
    resto: u32,
    sticky: Option<IntBool>,
    closed: Option<IntBool>,
    now: String,
    time: i64,
    name: String,
    trip: Option<String>,
    id: Option<String>,
    capcode: Option<String>,
    country: Option<String>,
    country_name: Option<String>,
    sub: Option<String>,
    com: Option<String>,
    tim: Option<i64>,
    filename: Option<String>,
    ext: Option<String>,
    fsize: Option<i32>,
    md5: Option<String>,
    w: Option<i32>,
    h: Option<i32>,
    tn_w: Option<i32>,
    tn_h: Option<i32>,
    filedeleted: Option<IntBool>,
    spoiler: Option<IntBool>,
    custom_spoiler: Option<u8>,
    omitted_posts: Option<i32>,
    omitted_images: Option<i32>,
    replies: Option<i32>,
    images: Option<i32>,
    bumplimit: Option<IntBool>,
    imagelimit: Option<IntBool>,
    last_modified: Option<i64>,
    tag: Option<String>,
    semantic_url: Option<String>,
    since4pass: Option<i8>,
    unique_ips: Option<i32>,
    m_img: Option<IntBool>
}

#[derive(Deserialize)]
struct FourPosts {
    posts: Vec<FourPost>
}

#[derive(Deserialize)]
struct FourPage {
    threads: Vec<FourPosts>
}

pub struct Fourchan;

impl super::ImageBoard for Fourchan {
	fn get_url(&self, board: Board, content: String) -> String {
		format!("https://a.4cdn.org/{}/{}", board, content)
    }

	fn get_last_thread(&self, board: Board) -> ThreadInfo {
        let url = self.get_url(board.clone(), "1.json".to_owned());
		let response = reqwest::blocking::get(url).unwrap();
		let deserialized: FourPage = serde_json::from_str(&response.text().unwrap()).unwrap();
        let post: FourPost = deserialized.threads[0].posts[0].clone();
        ThreadInfo {
			thread: Thread {board: board, id: post.no.to_string()},
			comment: parse_html(&post.com.unwrap_or("unknown".to_owned())),
			posts_count: post.replies.unwrap_or(0),
			timestamp: post.tim.unwrap_or(0)
		}
    }
    
	fn get_threads(&self, board: Board) -> Vec<ThreadInfo> {
        vec![
            ThreadInfo {
                thread: Thread { id: "foo".to_owned(), board: board },
                comment: "bar".to_owned(),
                posts_count: 420,
                timestamp: 69
            }
        ]
    }

	fn get_catalog(&self, board: Board) -> Vec<Catalog> {
        vec![
            Catalog {
                board: board,
                board_name: "fizz".to_owned(),
                comment: "buzz".to_owned(),
                email: "String".to_owned(),
                op: true,
                posts_count: 123,
                files_count: 321,
                files: vec![
                    File {
                        uri: "https://example.com".to_owned(),
                        thumbnail: "https://example1.com".to_owned(),
                        name: "example.jpg".to_owned(),
                        name_original: Some("Example.jpg".to_owned())
                    }
                ],
                timestamp: 123
            }
        ]
    }

	fn get_thread_posts(&self, thread: Thread) -> Vec<Post> {
        vec![
            Post {
                id: "id".to_owned(),
                comment: "comment".to_owned(),
                timestamp: 123,
                email: "email".to_owned(),
                files: vec![
                    File {
                        uri: "https://example.com".to_owned(),
                        thumbnail: "https://example1.com".to_owned(),
                        name: "example.jpg".to_owned(),
                        name_original: Some("Example.jpg".to_owned())
                    }
                ],
                op: true
            }
        ]
    }
}
