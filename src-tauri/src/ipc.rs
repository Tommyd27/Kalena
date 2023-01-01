use crate::prelude::*;
use crate::ctx::Ctx;
use crate::{error, Store};
use chrono::{Local, Timelike};
use tauri::{command, Wry, AppHandle};
use serde::{Serialize, Deserialize};
use std::fs;
use tokio::time::{sleep, Duration};


#[derive(Serialize, Clone)]
pub struct Player
{
	name : String,
	mmr : f32,
	id : String,
}



impl TryFrom<&str> for Player
{
	type Error = Error;
	fn try_from(player_str : &str) -> Result<Player>
	{
		if let [name, mmr, id] = player_str.split_terminator(",").collect::<Vec<&str>>()[..]
		{
			return Ok(Player{
				name : name.into(),
				mmr : mmr.parse::<f32>().unwrap(),
				id : id.into()
			})
		}
		Err(Error::PlayerParseError)
	}
}

#[derive(Deserialize, Debug)]
pub struct ReceivePlayer {
	pub id : String,
	pub name : String,
	pub generalStats : Vec<(String, i32)>
}

#[command]
pub async fn send_time_wake(time: String, connection: AppHandle<Wry>) -> bool{
   match Ctx::from_app(connection)
   {
	Ok(ctx) => {
		let result = Store::insert_time(time, ctx).await;
		result.is_ok()
	},
	Err(_) => {println!("go next"); false},
   }
}
  
#[command]
pub async fn need_date(connection: AppHandle<Wry>) -> bool
{
	match Ctx::from_app(connection)
   {
	Ok(ctx) => {
		let date = Store::fetch_string(ctx.get_store(), "date", "wakeup").await.unwrap_or("-".into());
		let today = Local::today().to_string();
		println!("{date} {today}");
		if date != Local::today().to_string() && Local::now().hour() > 4 {
			return true
		}
		false
	},
	Err(_) => true,
   }
}

#[command]
pub async fn fetch_players() -> Vec<Player>
{
	let path = "C:\\Users\\tom\\AppData\\Roaming\\bakkesmod\\bakkesmod\\data\\dejavu_k\\current_player_counters.txt";
	let players = match fs::read_to_string(path){
		Ok(s) => s,
		Err(_) => {sleep(Duration::from_secs(1)).await;
			fs::read_to_string(path).unwrap()
			//if fail unwrap should send message to frontend saying error should retry/ check file YEP
		}
	};

	players.split(";").filter(|x| !x.is_empty()).map(|p_string| Player::try_from(p_string).unwrap()).collect()
}

#[command]
pub async fn insert_players(playersInfo : Vec<ReceivePlayer>, connection: AppHandle<Wry>) -> bool {
	println!("{playersInfo:?}");
	match Ctx::from_app(connection) {
		Ok(ctx) => {
			Store::insert_players(playersInfo, ctx).await;
			true
		},
		Err(_) => false,
   }
}

#[command]
pub async fn fetch_stats(connection : AppHandle<Wry>) -> Vec<(String, String)> {
	Store::fetch_stats(Ctx::from_app(connection).unwrap()).await.unwrap()
}
#[command]
pub async fn add_stat(statToAdd : String, connection : AppHandle<Wry>) -> bool {
	match Ctx::from_app(connection) {
		Ok(ctx) => {
			Store::add_stat(statToAdd, ctx).await.unwrap_or(false)
		},
		Err(_) => false,
   }
}

#[command]
pub async fn delete_stat(stat_id : String, connection : AppHandle<Wry>) {
	println!("arrived {stat_id}");
	match Ctx::from_app(connection) {
		Ok(ctx) => {
			Store::del_stat(stat_id, ctx).await;
		},
		Err(_) => (),
   }
}


#[command]
pub async fn create_task()