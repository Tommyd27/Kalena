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
use crate::ipc::{send_time_wake};


mod prelude;
mod error;
mod store;
mod ipc;
mod ctx;
mod utils;


#[tokio::main]
async fn main() -> Result<()> {
	let store = Arc::new(Store::new().await?);
	let fetchedDate = Store::fetch_string(store.clone(), "date", "wakeup").await.unwrap_or("---".into());
  	tauri::Builder::default()
	.setup(|app|
	{
		let main_window = app.get_window("main").unwrap();

		main_window.listen("start-up", |event|{
			println!("Startup {:?}", event.payload());
		});

		let requireDay = move |x| Local::now().date().to_string() != x;
		println!("helpo");
		//move || if
		if requireDay(fetchedDate) 
		{
			println!("helo");
			main_window.emit("new-day", IntPayload {message: 0.into()});
		}
		else
		{
			println!("helllo");
			main_window.emit("new-day", IntPayload {message: 1});
		};
		
		Ok(())
	})
	.manage(store)
	.invoke_handler(tauri::generate_handler![
		send_time_wake,
	])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	
	Ok(())
}
