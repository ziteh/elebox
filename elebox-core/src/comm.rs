use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{DbError, EleboxError};

pub const ITEM_PART: &str = "part";
pub const ITEM_CAT: &str = "category";
pub const ITEM_PKG: &str = "package";
pub const ITEM_MFR: &str = "manufacturer";

pub trait Database<DI>: Send + Sync {
    fn init(&self) -> Result<(), DbError>;
    fn get_id(&self, name: &str) -> Result<String, DbError>;
    fn get(&self, id: &str) -> Result<DI, DbError>;
    fn list(&self) -> Result<Vec<DI>, DbError>;
    fn add(&self, item: &DI) -> Result<(), DbError>;
    fn update(&self, ori_id: &str, new_item: &DI) -> Result<(), DbError>;
    fn delete(&self, id: &str) -> Result<(), DbError>;
    fn check(&self) -> Result<(), DbError>;
}

pub trait Handler<T> {
    fn delete(&self, name: &str) -> Result<(), EleboxError>;
    fn add(&self, item: &T) -> Result<(), EleboxError>;
    fn update(&self, ori_name: &str, new_item: &T) -> Result<(), EleboxError>;
    fn get(&self, name: &str) -> Result<T, EleboxError>;
    fn list(&self) -> Result<Vec<T>, EleboxError>;
}

pub trait Transferable {
    fn export(&self, filename: &PathBuf) -> Result<(), EleboxError>;
    fn import(&self, filename: &PathBuf) -> Result<(), EleboxError>;
}

pub trait HumanReadable {
    fn write<T: Serialize>(filename: &PathBuf, items: Vec<T>) -> Result<(), ()>;
    fn read<T: for<'de> Deserialize<'de>>(filename: &PathBuf) -> Result<Vec<T>, ()>;
    fn check_extension(filename: &PathBuf) -> bool;
}
