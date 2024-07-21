

use crate::{DbError, EleboxError};

pub const ITEM_PART: &str = "part";
pub const ITEM_CAT: &str = "category";
pub const ITEM_PKG: &str = "package";
pub const ITEM_MFR: &str = "manufacturer";

pub trait Database<T> {
    fn init(&self) -> Result<(), DbError>;
    fn get_id(&self, name: &str) -> Result<String, DbError>;
    fn get(&self, id: &str) -> Result<T, DbError>;
    fn list(&self) -> Result<Vec<T>, DbError>;
    fn add(&self, item: &T) -> Result<(), DbError>;
    fn update(&self, ori_id: &str, new_item: &T) -> Result<(), DbError>;
    fn delete(&self, id: &str) -> Result<(), DbError>;
}

pub trait Manager<T> {
    fn init(&self) -> Result<(), EleboxError>;
    fn delete(&self, name: &str) -> Result<(), EleboxError>;
    fn add(&self, item: &T) -> Result<(), EleboxError>;
    fn update(&self, ori_name: &str, new_item: &T) -> Result<(), EleboxError>;
    fn get(&self, name: &str) -> Result<T, EleboxError>;
    fn list(&self) -> Result<Vec<T>, EleboxError>;
}

pub trait Transferable {
    fn export(&self, filename: &str) -> Result<(), EleboxError>;
    fn import(&self, filename: &str) -> Result<(), EleboxError>;
}
