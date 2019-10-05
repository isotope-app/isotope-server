use diesel::{
	r2d2::PoolError,
	result::{DatabaseErrorKind, Error as DieselError},
};
use serde_json::{Value as JsonValue};


#[derive(Fail, Debug)]
pub enum Error{
    #[fail(display = "Internal Server Error")]
	InternalServerError,	
	
	#[fail(display = "Unprocessable Entity: {}", _0)]
	UnprocessableEntity(JsonValue),
	
	#[fail(display = "Not Found: {}", _0)]
	NotFound(JsonValue),
}

impl From<PoolError> for Error {
    fn from(_error: PoolError) -> Self {
        Error::InternalServerError
    }
}

impl From<DieselError> for Error {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::UnprocessableEntity(json!({ "error": message }));
                }
                Error::InternalServerError
            }
            DieselError::NotFound => {
                Error::NotFound(json!({ "error": "requested record was not found" }))
            }
            _ => Error::InternalServerError,
        }
    }
}
