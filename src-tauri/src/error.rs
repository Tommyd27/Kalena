#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] surrealdb::Error),
}