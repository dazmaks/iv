use crate::types::{Board, ThreadInfo, CatalogThread, File};

use serde::{Serialize, Deserialize};
use html2md::parse_html;

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct ThreadsJSON {
    board: String,
    threads: Vec<ThreadShort>
}

#[derive(Serialize, Deserialize)]
struct AbuNews {
    date: String,
    num: i32,
    subject: String,
    views: i32
}

#[derive(Serialize, Deserialize)]
struct DvachFile {
    display_name: String,
    full_name: String,
    height: i32,
    md5: String,
    name: String,
    nsfw: bool,
    path: String,
    size: i32,
    thumbnail: String,
    tn_height: i32,
    tn_width: i32,
    file_type: i32,
}

#[derive(Serialize, Deserialize)]
struct DvachCatalogThread {
    banned: bool,
    closed: bool,
    comment: String,
    date: String,
    email: String,
    endless: bool,
    files: Vec<DvachFile>,
    files_count: i32,
    lasthit: i32,
    name: String,
    num: String,
    op: bool,
    parent: String,
    posts_count: i32,
    sticky: bool,
    subject: String,
    tags: String,
    timestamp: i64,
    trip: String
}

#[derive(Serialize, Deserialize)]
struct TopBoard {
    board: String,
    info: String,
    name: String
}
#[derive(Serialize, Deserialize)]
struct Catalog {
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
    enable_dices: bool,
    enable_flags: bool,
    enable_icons: bool,
    enable_images: bool,
    enable_likes: bool,
    enable_names: bool,
    enable_oekaki: bool,
    enable_posting: bool,
    enable_sage: bool,
    enable_shield: bool,
    enable_subject: bool,
    enable_thread_tags: bool,
    enable_trips: bool,
    enable_video: bool,
    filter: String,
    max_comment: i32,
    max_files_size: i32,
    news_abu: Vec<AbuNews>,
    threads: Vec<DvachCatalogThread>
}

pub struct Dvach;

impl super::ImageBoard for Dvach {
	fn get_last(&self, board: Board) -> ThreadInfo {
		let url: String = format!("https://2ch.hk/{}/threads.json", board);
		let response = reqwest::blocking::get(url).unwrap();
		let desir: ThreadsJSON = serde_json::from_str(&response.text().unwrap()).unwrap();
		ThreadInfo {
            board: desir.board,
            id: desir.threads[0].num.clone(),
            comment: parse_html(&desir.threads[0].comment).clone(),
            posts_count: desir.threads[0].posts_count,
            timestamp: desir.threads[0].timestamp
        }
	}
    fn get_catalog(&self, board: Board) -> Vec<CatalogThread> {
        let url: String = format!("https://2ch.hk/{}/catalog.json", board);
		let response = reqwest::blocking::get(url).unwrap();
		let desir: Catalog = serde_json::from_str(&response.text().unwrap()).unwrap();
		let catalog: Vec<CatalogThread> = desir.threads.into_iter().map(|e|
            CatalogThread {
                board: desir.board.clone(),
                board_name: desir.board_name.clone(),
                comment: e.comment,
                email: e.email,
                op: e.op,
                posts_count: e.posts_count,
                files_count: e.files_count,
                files: e.files.into_iter().map(|f| File {
                    uri: f.path,
                    thumbnail: f.thumbnail,
                    name: f.display_name,
                    name_original: f.full_name
                }).collect(),
                timestamp: e.timestamp
            }).collect();
        catalog
    }
}
