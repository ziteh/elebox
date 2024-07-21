use std::{
    fmt::Debug,
    io::Read,
    marker::PhantomData,
    path::PathBuf,
    str::{self, from_utf8},
};

use jammdb::{Bucket, Tx, DB};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{comm::*, DbError};

pub const PARTS_BUCKET: &str = "parts";
pub const PACKAGES_BUCKET: &str = "packages";
pub const MFR_BUCKET: &str = "manufacturers";
pub const CATEGORIES_BUCKET: &str = "categories";

pub trait Named {
    fn get_name(&self) -> String;
}

pub type Id = String;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum CustomFieldType {
    Normal,
    Link,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CustomField {
    pub field_type: CustomFieldType,
    pub name: String,
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Supplier {
    pub name: String,
    pub link: String,
    pub price: Option<f32>, // TODO: change to 'rust_decimal'?
    pub note: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct DbPart {
    pub name: String,
    pub quantity: u16,
    pub category_id: Id,
    pub package_id: Id,
    pub package_detail: String,
    pub mfr_id: Id,
    pub alias: String,
    pub description: String,
    pub location: String,
    pub mfr_no: String,
    pub datasheet_link: String,
    pub product_link: String,
    pub image_link: String,
    pub custom_fields: Vec<CustomField>,
    pub suppliers: Vec<Supplier>,
    pub starred: bool,
}

impl Named for DbPart {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbCategory {
    pub name: String,
    pub parent_id: Id,
    pub alias: String,
}

impl Named for DbCategory {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbPackage {
    pub pkg_type: String, // TODO enum
    pub name: String,
    pub alias: String,
}

impl Named for DbPackage {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbManufacturer {
    pub name: String,
    pub alias: String,
    pub url: String,
}

impl Named for DbManufacturer {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct JammDatabase {
    /// Database file path.
    path: PathBuf,

    /// Database bucket name.
    bucket: String,
}

impl JammDatabase {
    pub fn new(path: &str, bucket: &str) -> Self {
        Self {
            path: PathBuf::from(path),
            bucket: String::from(bucket),
        }
    }
}

// TODO return Err
impl<T> BaseDatabase<T> for JammDatabase
where
    T: Serialize + for<'de> Deserialize<'de> + Named,
{
    fn init(&self) -> Result<(), DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        tx.get_or_create_bucket(self.bucket.as_str()).unwrap();
        let _ = tx.commit().unwrap();
        Ok(())
    }

    fn add(&self, item: &T) -> Result<(), DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        let value = rmp_serde::to_vec(&item).unwrap();
        let id = Uuid::new_v4().to_string();
        bkt.put(id, value).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn update(&self, ori_id: &str, new_item: &T) -> Result<(), DbError> {
        let db = DB::open(&self.path)?;
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        let value = rmp_serde::to_vec(&new_item).unwrap();
        bkt.put(ori_id, value).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn get_id(&self, name: &str) -> Result<String, DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();

        let bkt = tx.get_bucket(self.bucket.as_str()).expect(&self.bucket);

        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice(data.kv().value()).unwrap();

            if &item.get_name() == name {
                let id = from_utf8(data.kv().key()).unwrap();
                return Ok(id.to_string());
            };
        }
        Err(DbError::NotExists(self.bucket.to_string())) // TODO
    }

    fn get(&self, id: &str) -> Result<T, DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        if let Some(data) = bkt.get(id) {
            return Ok(rmp_serde::from_slice(data.kv().value()).unwrap());
        }
        Err(DbError::NotExists(self.bucket.to_string())) // TODO
    }

    fn list(&self) -> Result<Vec<T>, DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        let mut items: Vec<T> = Vec::new();
        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice::<T>(data.kv().value()).unwrap();
            items.push(item);
        }
        Ok(items)
    }

    fn delete(&self, id: &str) -> Result<(), DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        bkt.delete(id).unwrap();
        let _ = tx.commit();
        Ok(())
    }
}

// TODO
impl From<jammdb::Error> for DbError {
    fn from(err: jammdb::Error) -> DbError {
        DbError::AccessFailed("jammdb error".to_string())
    }
}
