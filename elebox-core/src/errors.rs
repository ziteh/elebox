use core::fmt;
use std::error::Error;

#[derive(fmt::Debug)]
pub enum EleboxError {
    AlreadyExists(String, String),
    NotExists(String, String),
    InventoryShortage(String),
    DatabaseError(DbError),
}

impl Error for EleboxError {}

impl fmt::Display for EleboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EleboxError::AlreadyExists(ref item, ref name) => {
                write!(f, "{} {} already exists", item, name)
            }
            EleboxError::NotExists(ref item, ref name) => {
                write!(f, "{} {} does not exists", item, name)
            }
            EleboxError::InventoryShortage(ref name) => {
                write!(f, "Part {} not enough stock", name)
            }
            EleboxError::DatabaseError(ref error) => {
                write!(f, "database error {}", error)
            }
        }
    }
}

impl From<DbError> for EleboxError {
    fn from(err: DbError) -> EleboxError {
        match err {
            DbError::NotExists(name) => EleboxError::NotExists("".to_string(), name.to_string()), // TODO
            _ => EleboxError::DatabaseError(err),
        }
    }
}

#[derive(fmt::Debug)]
pub enum DbError {
    CannotOpenDb(String),
    CannotOpenBucket(String),
    AccessFailed(String),
    NotExists(String),
}

impl Error for DbError {}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::CannotOpenDb(ref name) => {
                write!(f, "cannot open or connect database {}", name)
            }
            DbError::CannotOpenBucket(ref name) => {
                write!(f, "cannot open bucket {}", name)
            }
            DbError::AccessFailed(ref name) => {
                write!(f, "{} access failed", name)
            }
            DbError::NotExists(ref name) => {
                write!(f, "{} does not exists", name)
            }
        }
    }
}

impl From<jammdb::Error> for DbError {
    fn from(err: jammdb::Error) -> DbError {
        DbError::AccessFailed("jammdb error".to_string()) // TODO
    }
}
