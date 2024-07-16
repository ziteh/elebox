use core::fmt;
use std::error::Error;

#[derive(fmt::Debug)]
pub enum EleboxError {
    AlreadyExists(String, String),
    NotExists(String, String),
    InventoryShortage(String),
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
        }
    }
}
