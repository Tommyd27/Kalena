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
use crate::ipc::{send_time_wake, need_date};
use tauri::Window;


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
	.setup(move |app|
	{
		let app_ = app.handle();
		app.listen_global("get_rocket_league_players", move |_e|{
			
		});
		Ok(())
	})
	.manage(store)
	.invoke_handler(tauri::generate_handler![
		send_time_wake,
		need_date
	])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	
	Ok(())
}
