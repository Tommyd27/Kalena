use std::collections::BTreeMap;
use std::str::FromStr;

use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};
use crate::prelude::*;
use crate::ctx::Ctx;
use crate::utils::XTake;
use x_takes::*;
use std::sync::Arc;
use chrono::{Local};
use crate::utils::map;
use crate::ipc::ReceivePlayer;
//mod error;

mod try_from;
mod x_takes;


pub struct Store
{
	ds : Datastore,
	ses: Session,
}
/*
Database Structure:

Tables:
	wakeup:
		date : str
		timeWake : str
		timeComputer : str
	
		rocketLeaguePlayers:

		rocketLeagueGames:

*/


impl Store
{
	pub async fn new() -> Result<Self>
	{
		//let ds = Datastore::new("file://temp.db").await?;
		let ds = Datastore::new("memory").await?;
		let ses = Session::for_db("appns", "appdb");
		Ok(Store {ds, ses})
	}
	pub async fn insert_time(time : String, handle : Arc<Ctx>) -> Result<()> {
		let store = handle.get_store();
		//let timeutc : DateTime<Utc> = DateTime::from_str(&time)?;
		let sql = "CREATE wakeup CONTENT $data";
		let cTime = Local::now();
		let data : BTreeMap<String, Value>= [
			("timeWake".into(), time.into()),
			("timeComputer".into(), cTime.time().to_string().into()),
			("date".into(), cTime.date().to_string().into())
		 ].into();
		let vars : BTreeMap<String, Value> = 
		[
			("data".into(), data.into())
		].into();

		let ress = store.ds.execute(sql, &store.ses, Some(vars), false).await;

		println!("{ress:?}");

		Ok(())
	}
	pub async fn fetch_string(store : Arc<Store>, field : &str, table : &str) -> Result<String>// -> Result<Object>
	{
		let sql = &format!("SELECT {field} FROM {table}");


		let ress = store.ds.execute(sql, &store.ses, None, true).await?.into_iter().next();//.result?.make_datetime();
		let out : Result<Object>  = W(ress.unwrap().result?.first()).try_into();
		println!("{out:?}");

		let p : Option<Result<String>> = out?.remove(field).map(|v| W(v).try_into());
		//println!("{ress:?}");
		println!("{p:?}");


		p.unwrap()
	}
	pub async fn insert_players(players : Vec<ReceivePlayer>, handle : Arc<Ctx>) -> Result<()>{

		let store = handle.get_store();

		let sql = "CREATE games CONTENT $data";
		for player in players {
			//2 + player
			let mut data : BTreeMap<String, Value>= [
				("playerID".into(), player.id.into()).into(),
				("name".into(), player.name.into()).into(),
			].into();
			for (statName, statValue) in player.generalStats {
				data.insert(statName.into(), statValue.into());
			}
			let vars : BTreeMap<String, Value> = [("data".into(), data.into())].into();
				
			let ress = store.ds.execute(sql, &store.ses, Some(vars), false).await;
			println!("{ress:?}");
		}

		Ok(())
	}
}