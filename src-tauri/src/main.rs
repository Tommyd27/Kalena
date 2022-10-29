#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use surrealdb::{Datastore, Session};
use crate::prelude::*;
use store::Store;
use std::sync::Arc;


mod prelude;
mod error;
mod store;



#[tokio::main]
async fn main() -> Result<()> {
	let store = Arc::new(Store::new().await?);

  	tauri::Builder::default()
	.manage(store)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	
	Ok(())
}
