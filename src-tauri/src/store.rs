use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};
use crate::prelude::*;
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
}