use std::{
    fmt::Debug,
    panic,
    path::PathBuf,
    str::{self, from_utf8},
};

use jammdb::DB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{comm::*, DbError};

pub const PARTS_BUCKET: &str = "parts";
pub const PACKAGES_BUCKET: &str = "packages";
pub const MFR_BUCKET: &str = "manufacturers";
pub const CATEGORIES_BUCKET: &str = "categories";

pub trait DatabaseItem {
    fn get_name(&self) -> String;
    fn get_bucket() -> String;
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

impl DatabaseItem for DbPart {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_bucket() -> String {
        String::from(PARTS_BUCKET)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbCategory {
    pub name: String,
    pub parent_id: Id,
    pub alias: String,
}

impl DatabaseItem for DbCategory {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_bucket() -> String {
        String::from(CATEGORIES_BUCKET)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbPackage {
    pub pkg_type: String, // TODO enum
    pub name: String,
    pub alias: String,
}

impl DatabaseItem for DbPackage {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_bucket() -> String {
        String::from(PACKAGES_BUCKET)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbManufacturer {
    pub name: String,
    pub alias: String,
    pub url: String,
}

impl DatabaseItem for DbManufacturer {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_bucket() -> String {
        String::from(MFR_BUCKET)
    }
}

pub struct JammDatabase {
    /// Database file path.
    path: PathBuf,
}

impl JammDatabase {
    pub fn new(path: &str) -> Self {
        Self {
            path: PathBuf::from(path),
        }
    }
}

// TODO return Err
impl<DI> Database<DI> for JammDatabase
where
    DI: Serialize + for<'de> Deserialize<'de> + DatabaseItem + Send + Sync,
{
    fn init(&self) -> Result<(), DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        tx.get_or_create_bucket(DI::get_bucket()).unwrap();
        let _ = tx.commit().unwrap();
        Ok(())
    }

    fn add(&self, item: &DI) -> Result<(), DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(DI::get_bucket()).unwrap();

        let value = rmp_serde::to_vec(&item).unwrap();
        let id = Uuid::new_v4().to_string();
        bkt.put(id, value).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn update(&self, ori_id: &str, new_item: &DI) -> Result<(), DbError> {
        let db = DB::open(&self.path)?;
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(DI::get_bucket()).unwrap();

        let value = rmp_serde::to_vec(&new_item).unwrap();
        bkt.put(ori_id, value).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn get_id(&self, name: &str) -> Result<String, DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();

        let bkt = tx.get_bucket(DI::get_bucket()).expect(&DI::get_bucket());

        for data in bkt.cursor() {
            let item: DI = rmp_serde::from_slice(data.kv().value()).unwrap();

            if &item.get_name() == name {
                let id = from_utf8(data.kv().key()).unwrap();
                return Ok(id.to_string());
            };
        }
        Err(DbError::NotExists(DI::get_bucket())) // TODO
    }

    fn get(&self, id: &str) -> Result<DI, DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(DI::get_bucket()).unwrap();

        if let Some(data) = bkt.get(id) {
            return Ok(rmp_serde::from_slice(data.kv().value()).unwrap());
        }
        Err(DbError::NotExists(DI::get_bucket())) // TODO
    }

    fn list(&self) -> Result<Vec<DI>, DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(DI::get_bucket()).unwrap();

        let mut items: Vec<DI> = Vec::new();
        for data in bkt.cursor() {
            let item: DI = rmp_serde::from_slice::<DI>(data.kv().value()).unwrap();
            items.push(item);
        }
        Ok(items)
    }

    fn delete(&self, id: &str) -> Result<(), DbError> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(DI::get_bucket()).unwrap();

        bkt.delete(id).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn check(&self) -> Result<(), DbError> {
        let result = panic::catch_unwind(|| {
            println!("Db");
            let db = DB::open(&self.path).unwrap();
            println!("Tx");
            let tx = db.tx(false).unwrap();
            println!("Bk");
            let bkt = tx.get_bucket(DI::get_bucket()).unwrap();
            println!("ok");
        });

        if result.is_err() {
            return Err(DbError::AccessFailed("".to_string()));
        }
        Ok(())
    }
}

// TODO
impl From<jammdb::Error> for DbError {
    fn from(_err: jammdb::Error) -> DbError {
        DbError::AccessFailed("jammdb error".to_string())
    }
}
