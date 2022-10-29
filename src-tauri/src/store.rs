use surrealdb::sql::{thing, Array, Datetime, Object, Value};
use surrealdb::{Datastore, Session};


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
	}
}