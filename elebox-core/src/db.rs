use std::{
    fmt::Debug,
    marker::PhantomData,
    path::PathBuf,
    str::{self, from_utf8},
};

use jammdb::{Bucket, Tx, DB};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::comm::*;

pub const PARTS_BUCKET: &str = "parts";
pub const PACKAGES_BUCKET: &str = "packages";
pub const MFR_BUCKET: &str = "manufacturers";
pub const CATEGORIES_BUCKET: &str = "categories";

pub trait BaseDatabase<T> {
    fn init(&self) -> Result<(), ()>;
    fn get_id(&self, name: &str) -> Option<String>;
    fn get(&self, id: &str) -> Option<T>;
    fn list(&self) -> Vec<T>;
    fn add(&self, item: &T) -> Result<(), ()>;
    fn update(&self, ori_id: &str, new_item: &T) -> Result<(), ()>;
    fn delete(&self, id: &str) -> Result<(), ()>;
}

pub struct JammDatabase {
    path: PathBuf,
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

impl<T> BaseDatabase<T> for JammDatabase
where
    T: Serialize + for<'de> Deserialize<'de> + DbItem,
{
    fn init(&self) -> Result<(), ()> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        tx.get_or_create_bucket(self.bucket.as_str()).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn add(&self, item: &T) -> Result<(), ()> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        let value = rmp_serde::to_vec(&item).unwrap();
        let id = Uuid::new_v4().to_string();
        bkt.put(id, value).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn update(&self, ori_id: &str, new_item: &T) -> Result<(), ()> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        let value = rmp_serde::to_vec(&new_item).unwrap();
        bkt.put(ori_id, value).unwrap();
        let _ = tx.commit();
        Ok(())
    }

    fn get_id(&self, name: &str) -> Option<String> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice(data.kv().value()).unwrap();

            if &item.get_name() == name {
                let id = from_utf8(data.kv().key()).unwrap();
                return Some(id.to_string());
            };
        }
        None
    }

    fn get(&self, id: &str) -> Option<T> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        if let Some(data) = bkt.get(id) {
            return Some(rmp_serde::from_slice(data.kv().value()).unwrap());
        }
        None
    }

    fn list(&self) -> Vec<T> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        let mut items: Vec<T> = Vec::new();
        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice::<T>(data.kv().value()).unwrap();
            items.push(item);
        }
        items
    }

    fn delete(&self, id: &str) -> Result<(), ()> {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(self.bucket.as_str()).unwrap();

        bkt.delete(id).unwrap();
        let _ = tx.commit();
        Ok(())
    }
}
