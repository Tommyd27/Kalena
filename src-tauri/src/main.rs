#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use chrono::Local;
use surrealdb::{Datastore, Session};
use tauri::Manager;
use crate::prelude::*;
use store::Store;
use std::sync::Arc;
use crate::ipc::{send_time_wake, need_date, fetch_players, insert_players, fetch_stats, add_stat};
use tauri::Window;
use std::thread;
use std::time;
use std::fs;
use serde::Serialize;
 

mod prelude;
mod error;
mod store;
mod ipc;
mod ctx;
mod utils;




#[tokio::main]
async fn main() -> Result<()> {
	let store = Arc::new(Store::new().await?);
	
  	tauri::Builder::default()
	/*.setup(move |app|
	{
		Ok(())
	})*/
	.manage(store)
	.invoke_handler(tauri::generate_handler![
		send_time_wake,
		need_date,
		fetch_players,
		insert_players,
		fetch_stats,
		add_stat,
	])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	
	Ok(())
}
