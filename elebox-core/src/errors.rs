use core::fmt;
use std::error::Error;

#[derive(fmt::Debug)]
pub enum EleboxError {
    PartAlreadyExists(String),
    PartNotExists(String),
    PartInventoryShortage(String),
}

impl Error for EleboxError {}

impl fmt::Display for EleboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EleboxError::PartAlreadyExists(ref name) => write!(f, "Part {} already exists", name),
            EleboxError::PartNotExists(ref name) => write!(f, "Part {} does not exists", name),
            EleboxError::PartInventoryShortage(ref name) => {
                write!(f, "Part {} not enough stock", name)
            }
        }
    }
}
