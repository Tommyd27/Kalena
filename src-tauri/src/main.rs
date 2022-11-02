#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use surrealdb::{Datastore, Session};
use crate::prelude::*;
use store::Store;
use std::sync::Arc;
use crate::ipc::send_time_wake;


mod prelude;
mod error;
mod store;
mod ipc;
mod ctx;



#[tokio::main]
async fn main() -> Result<()> {
	let store = Arc::new(Store::new().await?);

  	tauri::Builder::default()
	.manage(store)
	.invoke_handler(tauri::generate_handler![
		send_time_wake
	])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	
	Ok(())
}
