use crate::prelude::*;
use crate::Store;
use tauri::{command, State};

#[command]
pub async fn hello(time: String, connection: State<Store>) -> String {
   format!("hello {}", time)
}