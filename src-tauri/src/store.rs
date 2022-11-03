use std::collections::BTreeMap;

use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};
use crate::prelude::*;
use crate::ctx::Ctx;
use std::sync::Arc;
//mod error;

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
	pub async fn insertTime(time : String, handle : Arc<Ctx>)
	{
		let store = handle.get_store();

		let sql = "CREATE wakeup CONTENT $data";

		let data : BTreeMap<String, Value>= [
			("time".into(), time.into()),
		 ].into();
		let vars : BTreeMap<String, Value> = 
		[
			("data".into(), data.into())
		].into();

		let ress = store.ds.execute(sql, &store.ses, Some(vars), false).await;

		println!("{ress:?}");
	}
	pub async fn fetchLatestTime(store : Arc<Store>) -> String
	{
		let sql = "SELECT * FROM wakeup LIMIT 1";

		let ress = store.ds.execute(sql, &store.ses, None, false).await;

		let first_res = ress.into_iter().next().expect("No response");

		let out = first_res.first();

		match out
		{
			Some(result) => {
										let out_ = &result.result;
										println!("{out_:?}");
		},
			None => {println!("Go next");}
		}
		println!("{out:?}");
		"Helllo".into()
	}
}