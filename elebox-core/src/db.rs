use std::{
    fmt::Debug,
    str::{self, from_utf8},
};

use jammdb::DB;
// use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// use crate::{Category, EleboxError};

trait DbItem {
    fn get_name(&self) -> String;
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbPart {
    pub name: String,
    pub category_id: String,
    pub quantity: u16,
}

impl DbItem for DbPart {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub name: String,
    pub category: String,
    pub quantity: u16,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbCategory {
    pub name: String,
    pub parent_id: String,
}

impl DbItem for DbCategory {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

// const DEFAULT_DATABASE_PATH: &str = "./elebox.db";
const PARTS_BUCKET: &str = "parts";
const CATEGORIES_BUCKET: &str = "catrgories";

pub struct Datebase {
    path: String,
}

impl Datebase {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn init(&self) {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();

        tx.get_or_create_bucket(PARTS_BUCKET).unwrap();
        tx.get_or_create_bucket(CATEGORIES_BUCKET).unwrap();
        let _ = tx.commit();
    }

    fn add_item<T>(&self, bucket: &str, item: &T)
    where
        T: Serialize,
    {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        let value = rmp_serde::to_vec(&item).unwrap();
        let id = Uuid::new_v4().to_string();
        bkt.put(id, value).unwrap();
        let _ = tx.commit();
    }

    pub fn add_part(&self, part: &DbPart) {
        self.add_item::<DbPart>(PARTS_BUCKET, part)
    }

    pub fn add_category(&self, category: &DbCategory) {
        self.add_item::<DbCategory>(CATEGORIES_BUCKET, category);
    }

    fn get_item_id<T>(&self, bucket: &str, name: &str) -> Option<String>
    where
        T: for<'a> Deserialize<'a> + DbItem,
    {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice(data.kv().value()).unwrap();

            if &item.get_name() == name {
                let id = from_utf8(data.kv().key()).unwrap();
                return Some(id.to_string());
            };
        }
        return None;
    }

    pub fn get_part_id(&self, name: &str) -> Option<String> {
        self.get_item_id::<DbPart>(PARTS_BUCKET, name)
    }

    pub fn get_category_id(&self, name: &str) -> Option<String> {
        self.get_item_id::<DbCategory>(CATEGORIES_BUCKET, name)
    }

    fn get_item<T>(&self, bucket: &str, id: &str) -> Option<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        if let Some(data) = bkt.get(id) {
            return Some(rmp_serde::from_slice(data.kv().value()).unwrap());
        }
        return None;
    }

    pub fn get_part_from_id(&self, id: &str) -> Option<DbPart> {
        self.get_item::<DbPart>(PARTS_BUCKET, id)
    }
    pub fn get_category_from_id(&self, id: &str) -> Option<DbCategory> {
        self.get_item::<DbCategory>(CATEGORIES_BUCKET, id)
    }

    fn get_items<T>(&self, bucket: &str) -> Vec<T>
    where
        T: for<'a> Deserialize<'a>,
    {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        let mut items: Vec<T> = Vec::new();
        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice(data.kv().value()).unwrap();
            items.push(item);
        }

        return items;
    }

    pub fn get_parts(&self) -> Vec<DbPart> {
        self.get_items::<DbPart>(PARTS_BUCKET)
    }

    pub fn get_categories(&self) -> Vec<DbCategory> {
        self.get_items::<DbCategory>(CATEGORIES_BUCKET)
    }

    fn delete_item(&self, bucket: &str, id: &str) -> String {
        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        // assert!(bkt.get_kv(id).is_some());

        let res = bkt.delete(id);
        //    let _ = tx.commit();
        match res {
            Err(e) => return e.to_string(),
            _ => return "ok".to_string(),
        }

        // assert!(bkt.get_kv(id).is_none());
    }

    pub fn delete_part(&self, id: &str) -> String {
        self.delete_item(PARTS_BUCKET, id)
    }

    pub fn delete_category(&self, id: &str) -> String {
        self.delete_item(CATEGORIES_BUCKET, id)
    }

    pub fn update_part(&self, name: &str, part: &DbPart) {
        let id = self.get_part_id(name);
        if id.is_none() {
            return;
        }

        let db = DB::open(&self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(PARTS_BUCKET).unwrap();

        let value = rmp_serde::to_vec(&part).unwrap();
        bkt.put(id.unwrap(), value).unwrap();
        let _ = tx.commit();
    }
}
