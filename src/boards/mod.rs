pub mod dvach;
pub mod fourchan;

use crate::types::{Board, ThreadInfo, Catalog, Thread, Post};
use serde::de::{self, Deserialize, Deserializer, Unexpected};

#[derive(Clone)]
pub struct IntBool(bool);

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

pub trait ImageBoard {
	fn get_url(&self, board: Board, content: String) -> String;
	fn get_last_thread(&self, board: Board) -> ThreadInfo;
	fn get_threads(&self, board: Board) -> Vec<ThreadInfo>;
	fn get_catalog(&self, board: Board) -> Vec<Catalog>;
	fn get_thread_posts(&self, thread: Thread) -> Vec<Post>;
}
