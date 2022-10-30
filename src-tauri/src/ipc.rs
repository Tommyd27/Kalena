use crate::prelude::*;
use tauri::{command};

#[command]
pub async fn hello(time: String) -> String {
   format!("hello {}", time)
}