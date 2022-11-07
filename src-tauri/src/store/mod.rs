use std::collections::BTreeMap;
use std::str::FromStr;

use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};
use crate::ipc::fetch_latest_time;
use crate::prelude::*;
use crate::ctx::Ctx;
use crate::utils::XTake;
use x_takes::*;
use std::sync::Arc;
use chrono::{Local, DateTime, Utc};
//mod error;

mod try_from;
mod x_takes;


pub struct Store
{
	ds : Datastore,
	ses: Session
}

impl Store
{
	pub async fn new() -> Result<Self>
	{
		let ds = Datastore::new("file://temp.db").await?;
		let ses = Session::for_db("appns", "appdb");
		Ok(Store {ds, ses})
	}
	pub async fn insertTime(time : String, handle : Arc<Ctx>) -> Result<()>
	{
		let store = handle.get_store();
		//let timeutc : DateTime<Utc> = DateTime::from_str(&time)?;
		let sql = "CREATE wakeup CONTENT $data";
		let data : BTreeMap<String, Value>= [
			("timeWake".into(), DateTime::from_str(&time).unwrap().into()),
			("timeComputer".into(), Utc::now().into())
		 ].into();
		let vars : BTreeMap<String, Value> = 
		[
			("data".into(), data.into())
		].into();

		let ress = store.ds.execute(sql, &store.ses, Some(vars), false).await;

		println!("{ress:?}");

		Ok(())
	}//
	pub async fn fetchLatestTime(store : Arc<Store>) -> Result<String>// -> Result<Object>
	{
		let sql = "SELECT date FROM wakeup LIMIT 1";

		let ress = store.ds.execute(sql, &store.ses, None, true).await?.into_iter().next();//.result?.make_datetime();
		let out : Result<Object>  = W(ress.unwrap().result?.first()).try_into();
		let p : Option<Result<String>> = out?.remove("time").map(|v| W(v).try_into());
		//println!("{ress:?}");
		println!("{p:?}");

		
		//let first_res = ress.into_iter().next().expect("Did not get a response");
		//let smthing: Result<Object> = W(first_res.result?.first()).try_into();

		//smthing

		Ok("hello".into())
	}

}