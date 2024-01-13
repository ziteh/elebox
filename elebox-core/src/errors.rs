use core::fmt;
use std::error::Error;

#[derive(fmt::Debug)]
pub enum EleboxError {
    AlreadyExists(String),
    NotExists(String),
    InventoryShortage(String),
}

impl Error for EleboxError {}

impl fmt::Display for EleboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EleboxError::AlreadyExists(ref name) => {
                write!(f, "Item \"{}\" already exists", name)
            }
            EleboxError::NotExists(ref name) => write!(f, "Item \"{}\" does not exists", name),
            EleboxError::InventoryShortage(ref name) => {
                write!(f, "Part \"{}\" not enough stock", name)
            }
        }
    }
}
