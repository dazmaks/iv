use crate::boards::IntBool;
use crate::types::{Board, Catalog, File, Post, Thread, ThreadInfo};

use html2md::parse_html;

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
    views: i32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ThreadsJSON {
    board: String,
    threads: Vec<ThreadShort>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct AbuNews {
    date: String,
    num: i32,
    subject: String,
    views: i32,
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
    sticky: i8,
    subject: String,
    tags: String,
    timestamp: i64,
    trip: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TopBoard {
    board: String,
    info: String,
    name: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CatalogResp {
    #[serde(rename = "Board")]
    board: Board,
    #[serde(rename = "BoardInfo")]
    board_info: String,
    #[serde(rename = "BoardInfoOuter")]
    board_info_outer: String,
    #[serde(rename = "BoardName")]
    board_name: String,
    advert_bottom_image: Option<String>,
    advert_bottom_link: Option<String>,
    advert_mobile_image: Option<String>,
    advert_mobile_link: Option<String>,
    advert_top_image: Option<String>,
    advert_top_link: Option<String>,
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
    top: Vec<TopBoard>,
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
    sticky: i8,
    subject: String,
    timestamp: i64,
    trip: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ThreadRespThread {
    posts: Vec<ThreadRespPost>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ThreadResp {
    #[serde(rename = "Board")]
    board: Board,
    #[serde(rename = "BoardInfo")]
    board_info: String,
    #[serde(rename = "BoardInfoOuter")]
    board_info_outer: String,
    #[serde(rename = "BoardName")]
    board_name: String,
    advert_bottom_image: Option<String>,
    advert_bottom_link: Option<String>,
    advert_mobile_image: Option<String>,
    advert_mobile_link: Option<String>,
    advert_top_image: Option<String>,
    advert_top_link: Option<String>,
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
    top: Vec<TopBoard>,
}

pub struct Dvach;

impl super::ImageBoard for Dvach {
    fn get_url(&self, board: Board, content: String) -> String {
        format!("https://2ch.hk/{}/{}", board, content)
    }

    fn get_last_thread(&self, board: Board) -> ThreadInfo {
        let url = self.get_url(board, "threads.json".to_owned());
        let response = reqwest::blocking::get(url).unwrap();
        let deserialized: ThreadsJSON = serde_json::from_str(&response.text().unwrap()).unwrap();
        ThreadInfo {
            thread: Thread {
                board: deserialized.board,
                id: deserialized.threads[0].num.clone(),
            },
            comment: parse_html(&deserialized.threads[0].comment),
            posts_count: deserialized.threads[0].posts_count,
            timestamp: deserialized.threads[0].timestamp,
        }
    }

    fn get_threads(&self, board: Board) -> Vec<ThreadInfo> {
        let url = self.get_url(board, "threads.json".to_owned());
        let response = reqwest::blocking::get(url).unwrap();
        let deserialized: ThreadsJSON = serde_json::from_str(&response.text().unwrap()).unwrap();
        let threads: Vec<ThreadInfo> = deserialized
            .threads
            .into_iter()
            .map(|t| ThreadInfo {
                thread: Thread {
                    board: deserialized.board.clone(),
                    id: t.num,
                },
                comment: t.comment,
                posts_count: t.posts_count,
                timestamp: t.timestamp,
            })
            .collect();
        threads
    }

    fn get_catalog(&self, board: Board) -> Vec<Catalog> {
        let url = self.get_url(board, "catalog.json".to_owned());
        let response = reqwest::blocking::get(url).unwrap();
        let deserialized: CatalogResp = serde_json::from_str(&response.text().unwrap()).unwrap();
        let catalog: Vec<Catalog> = deserialized
            .threads
            .into_iter()
            .map(|e| Catalog {
                board: deserialized.board.clone(),
                board_name: deserialized.board_name.clone(),
                comment: parse_html(&e.comment),
                email: e.email,
                op: e.op.0,
                posts_count: e.posts_count,
                files_count: e.files_count,
                files: e
                    .files
                    .into_iter()
                    .map(|f| File {
                        uri: f.path,
                        thumbnail: f.thumbnail,
                        name: f.displayname,
                        name_original: f.fullname,
                    })
                    .collect(),
                timestamp: e.timestamp,
            })
            .collect();
        catalog
    }

    fn get_thread_posts(&self, thread: Thread) -> Vec<Post> {
        let url = self.get_url(thread.board, format!("res/{}.json", thread.id));
        let response = reqwest::blocking::get(url).unwrap();
        let deserialized: ThreadResp = serde_json::from_str(&response.text().unwrap()).unwrap();
        let posts = deserialized.threads[0]
            .posts
            .clone()
            .into_iter()
            .map(|p| Post {
                id: p.num.to_string(),
                comment: parse_html(&p.comment),
                timestamp: p.timestamp,
                email: p.email,
                files: p
                    .files
                    .into_iter()
                    .map(|f| File {
                        uri: f.path,
                        thumbnail: f.thumbnail,
                        name: f.name,
                        name_original: f.fullname,
                    })
                    .collect(),
                op: p.op.0,
            })
            .collect();
        posts
    }
}
