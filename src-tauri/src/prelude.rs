pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub struct W<T>(pub T);

#[derive(Clone, serde::Serialize)]
pub struct StrPayload{
	pub message : String
}
#[derive(Clone, serde::Serialize)]
pub struct IntPayload{
	pub message : i64
}