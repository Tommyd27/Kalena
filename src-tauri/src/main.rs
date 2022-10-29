#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


use surrealdb::{Datastore, Session, Error};



#[tokio::main]
async fn main() -> Result<(), Error> {
	let ds = Datastore::new("file://temp.db").await?;
	let session = Session::for_db("appns", "appdb");

  	tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
	Ok(())
}
