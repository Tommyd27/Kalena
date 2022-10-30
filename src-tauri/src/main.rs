#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use surrealdb::{Datastore, Session};
use crate::prelude::*;
use store::Store;
use std::sync::Arc;
use ipc::hello;

mod prelude;
mod error;
mod store;
mod ipc;



#[tokio::main]
async fn main() -> Result<()> {
	let store = Arc::new(Store::new().await?);

  	tauri::Builder::default()
	.manage(store)
	.invoke_handler(tauri::generate_handler![
		hello
	])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	
	Ok(())
}
