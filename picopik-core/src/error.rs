use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DieselError},
};
use serde_json::{Value as JsonValue};
use bcrypt::{
    BcryptError
};


#[derive(Fail, Debug)]
pub enum Error{
    #[fail(display = "Internal Server Error")]
    InternalServerError,    
    
    #[fail(display = "Database error {:?}", _0)]
    DbPoolError(PoolError),
    
    #[fail(display = "Diesel error {:?}", _0)]
    DieselError(DieselError),
    
    #[fail(display = "Unprocessable Entity: {}", _0)]
    UnprocessableEntity(JsonValue),
    
    #[fail(display = "Not Found: {}", _0)]
    NotFound(JsonValue),
    
    #[fail(display = "Failed to encrypt:")]
    EncryptionError,
    
    #[fail(display = "generic error")]
    GenericError
}

//impl From<()> for Error{
//  fn from(e::<()>) ->Self{)
//      Error::DbPoolError(e)
//  }
//}

pub struct GenericError{
    pub value: String
}

impl From<()> for Error{
    fn from(_error: ()) -> Self{
        Error::GenericError
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Error::DbPoolError(error)
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

impl From<BcryptError> for Error{
    fn from(bcrypt: BcryptError) -> Self{
        match bcrypt{
            _ => Error::EncryptionError,
            
        }
    }
}