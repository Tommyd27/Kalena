use std::collections::BTreeMap;
use std::str::FromStr;

use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session, Response};
use crate::prelude::*;
use crate::ctx::Ctx;
use crate::utils::{XTake, XTakeVal};
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
		let ds = Datastore::new("file://database_one").await?;
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

		println!("here {ress:?}");

		Ok(())
	}
	pub async fn fetch_string(store : Arc<Store>, field : &str, table : &str) -> Result<String>// -> Result<Object>
	{
		let sql = &format!("SELECT {field} FROM {table} ORDER BY date DESC LIMIT 1");


		let ress = store.ds.execute(sql, &store.ses, None, true).await?.into_iter().next();//.result?.make_datetime();
		println!("{ress:?}");
		let out : Result<Object>  = W(ress.unwrap().result?.first()).try_into();

		let p : Option<Result<String>> = out?.remove(field).map(|v| W(v).try_into());


		p.unwrap()
	}
	pub async fn fetch_stats(handle : Arc<Ctx>) -> Result<Vec<(String, String)>>// -> Result<Object>
	{
		let store = handle.get_store();


		let sql = &format!("SELECT * FROM statsToTrack");


		let ress = store.ds.execute(sql, &store.ses, None, true).await?;
		let out : Vec<(String, String)> = Self::into_iter_objects(ress)?.map(|obj| {
			let obj = obj.unwrap();
			let name = obj.get("name").unwrap().to_string().replace('"', "");
			let id = obj.get("id").unwrap().to_string();
			(id, name)
		}).collect();


		Ok(out)
	}
	pub async fn del_stat(stat : String, handle : Arc<Ctx>) -> Result<bool>// -> Result<Object>
	{
		let store = handle.get_store();

		let sql = &format!("DELETE {}", stat);


		let ress = store.ds.execute(sql, &store.ses, None, false).await?;//.result?.make_datetime();
		println!("{ress:?}");
		/*let out : Result<Object>  = W(ress.unwrap().result?.first()).try_into();

		let p : Option<Result<String>> = out?.remove(field).map(|v| W(v).try_into());*/


		Ok(true)
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

	fn into_iter_objects(ress : Vec<Response>) -> Result<impl Iterator<Item = Result<Object>>> {
		let res = ress.into_iter().next().map(|rp : Response| rp.result).transpose()?;
		match res {
			Some(Value::Array(arr)) => {
				let it = arr.into_iter().map(|v| match v {
					Value::Object(object) => Ok(object),
					_ => Err(Error::SurrealC("A record was not an object"))
				});
				Ok(it)
			},
			_ => Err(Error::SurrealC("No records found"))
		}
	}

	pub async fn add_stat(stat : String, handle : Arc<Ctx>) -> Result<bool>// -> Result<Object>
	{
		let store = handle.get_store();

		let data : BTreeMap<String, Value>= [
				("name".into(), stat.into()).into(),
			].into();
		let sql = &format!("CREATE statsToTrack CONTENT $data");
		let vars : BTreeMap<String, Value> = [("data".into(), data.into())].into();


		let ress = store.ds.execute(sql, &store.ses, Some(vars), false).await?;//.result?.make_datetime();
		println!("{ress:?}");
		/*let out : Result<Object>  = W(ress.unwrap().result?.first()).try_into();

		let p : Option<Result<String>> = out?.remove(field).map(|v| W(v).try_into());*/


		Ok(true)
	}

	pub async fn insert_values(handle : Arc<Ctx>, table : &str, values : BTreeMap<String, Value>) -> Result<()> {
		let store = handle.get_store();
		println!("{values:?}");
		let vars : BTreeMap<String, Value> = [
			("th".into(), thing(table)?.into()),
			("data".into(), values.into())
			].into();
		println!("in insert values");
		
		println!("{vars:?}");
		let sql = "CREATE $th CONTENT $data";
		let ress = store.ds.execute(sql, &store.ses, Some(vars), false).await?;//.result?.make_datetime();
		println!("in insert vasdfdsgdsfghlues");
		println!("{ress:?}");
		Ok(())
	}
}