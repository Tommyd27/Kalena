use crate::prelude::*;
use crate::ctx::Ctx;
use crate::{error, Store};
use tauri::{command, Wry, AppHandle};

#[command]
pub async fn send_time_wake(time: String, connection: AppHandle<Wry>) {
   match Ctx::from_app(connection)
   {
	Ok(ctx) => {Store::insert_time(time, ctx).await;	},
	Err(_) => println!("go next"),
   }
}

#[command]
pub async fn fetch_latest_time(connection: AppHandle<Wry>)
{
	match Ctx::from_app(connection)
   {
	Ok(ctx) => {Store::fetch_string(ctx.get_store(), "date", "wakeup").await;	},
	Err(_) => println!("go next"),
   }
}