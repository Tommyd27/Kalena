#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	Surreal(#[from] surrealdb::Error),

	#[error("Parse Error")]
	String(#[from] std::string::ParseError),

	#[error("Fail to get Ctx")]
	CtxFail,

	#[error("Value not of type '{0}'")]
	XValueNotOfType(&'static str),

	#[error("Property '{0}' not found")]
	XPropertyNotFound(String),

	#[error("Error Parsing Value to Player")]
	PlayerParseError,
}