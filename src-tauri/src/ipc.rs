use crate::prelude::*;
use crate::ctx::Ctx;
use crate::{error, Store};
use tauri::{command, Wry, AppHandle};

#[command]
pub async fn send_time_wake(time: String, connection: AppHandle<Wry>) {
   match Ctx::from_app(connection)
   {
	Ok(ctx) => {Store::insertTime(time, ctx).await;	},
	Err(_) => println!("go next"),
   }
}

#[command]
pub async fn fetch_latest_time(connection: AppHandle<Wry>)
{
	match Ctx::from_app(connection)
   {
	Ok(ctx) => {Store::fetchLatestTime(ctx.get_store()).await;	},
	Err(_) => println!("go next"),
   }
}