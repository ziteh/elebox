use std::{
    fmt::Debug,
    str::{self, from_utf8},
};

use jammdb::DB;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Id = String;

trait DbItem {
    fn get_name(&self) -> String;
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbPart {
    pub name: String,
    pub quantity: u16,
    pub category_id: Id,
    pub package_id: Id,
    pub mfr_id: Id,
    pub alias: String,
    pub description: String,
    pub cost: f32,
    pub location: String,
    pub mfr_no: String,
    pub mouser_no: String,
    pub digikey_no: String,
    pub datasheet_url: String,
    pub product_url: String,
    pub image_url: String,
    pub suppliers: String,
}

impl DbItem for DbPart {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbCategory {
    pub name: String,
    pub parent_id: Id,
}

impl DbItem for DbCategory {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbPackage {
    pub pkg_type: String,
    pub name: String,
    pub alias: String,
}

impl DbItem for DbPackage {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbManufacturer {
    pub name: String,
    pub alias: String,
    pub url: String,
}

impl DbItem for DbManufacturer {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

const PARTS_BUCKET: &str = "parts";
const CATEGORIES_BUCKET: &str = "categories";
const PACKAGES_BUCKET: &str = "packages";
const MFR_BUCKET: &str = "manufacturers";
const STAR_BUCKET: &str = "stars";

pub trait Database {
    fn init(&self);

    fn add_part(&self, part: &DbPart);
    fn add_category(&self, category: &DbCategory);
    fn add_package(&self, package: &DbPackage);
    fn add_mfr(&self, mfr: &DbManufacturer);

    fn get_part_id(&self, name: &str) -> Option<String>;
    fn get_category_id(&self, name: &str) -> Option<String>;
    fn get_package_id(&self, name: &str) -> Option<String>;
    fn get_mfr_id(&self, name: &str) -> Option<String>;

    fn get_part_from_id(&self, id: &str) -> Option<DbPart>;
    fn get_category_from_id(&self, id: &str) -> Option<DbCategory>;
    fn get_package_from_id(&self, id: &str) -> Option<DbPackage>;
    fn get_mfr_from_id(&self, id: &str) -> Option<DbManufacturer>;

    fn get_parts(&self) -> Vec<DbPart>;
    fn get_categories(&self) -> Vec<DbCategory>;
    fn get_packages(&self) -> Vec<DbPackage>;
    fn get_mfrs(&self) -> Vec<DbManufacturer>;

    fn delete_part(&self, id: &str) -> String;
    fn delete_category(&self, id: &str) -> String;
    fn delete_package(&self, id: &str) -> String;
    fn delete_mfr(&self, id: &str) -> String;

    fn update_part(&self, name: &str, part: &DbPart);
}

pub struct JammDatabase<'a> {
    path: &'a str,
}

impl<'a> JammDatabase<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { path }
    }

    fn add_item<T>(&self, bucket: &str, item: &T)
    where
        T: Serialize,
    {
        let db = DB::open(self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        let value = rmp_serde::to_vec(&item).unwrap();
        let id = Uuid::new_v4().to_string();
        bkt.put(id, value).unwrap();
        let _ = tx.commit();
    }

    fn get_item_id<T>(&self, bucket: &str, name: &str) -> Option<String>
    where
        T: for<'b> Deserialize<'b> + DbItem,
    {
        let db = DB::open(self.path).unwrap();
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

    fn get_item<T>(&self, bucket: &str, id: &str) -> Option<T>
    where
        T: for<'b> Deserialize<'b>,
    {
        let db = DB::open(self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        if let Some(data) = bkt.get(id) {
            return Some(rmp_serde::from_slice(data.kv().value()).unwrap());
        }
        return None;
    }

    fn get_items<T>(&self, bucket: &str) -> Vec<T>
    where
        T: for<'b> Deserialize<'b>,
    {
        let db = DB::open(self.path).unwrap();
        let tx = db.tx(false).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        let mut items: Vec<T> = Vec::new();
        for data in bkt.cursor() {
            let item: T = rmp_serde::from_slice::<T>(data.kv().value()).unwrap();
            items.push(item);
        }

        return items;
    }

    fn delete_item(&self, bucket: &str, id: &str) -> String {
        let db = DB::open(self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(bucket).unwrap();

        bkt.delete(id).unwrap();
        let _ = tx.commit();

        "done".to_string() // TODO
    }
}

impl<'a> Database for JammDatabase<'a> {
    fn init(&self) {
        let db = DB::open(self.path).unwrap();
        let tx = db.tx(true).unwrap();

        tx.get_or_create_bucket(PARTS_BUCKET).unwrap();
        tx.get_or_create_bucket(CATEGORIES_BUCKET).unwrap();
        tx.get_or_create_bucket(PACKAGES_BUCKET).unwrap();
        tx.get_or_create_bucket(MFR_BUCKET).unwrap();
        let _ = tx.commit();
    }

    fn add_part(&self, part: &DbPart) {
        self.add_item::<DbPart>(PARTS_BUCKET, part)
    }

    fn add_category(&self, category: &DbCategory) {
        self.add_item::<DbCategory>(CATEGORIES_BUCKET, category);
    }

    fn add_package(&self, package: &DbPackage) {
        self.add_item::<DbPackage>(PACKAGES_BUCKET, package);
    }

    fn add_mfr(&self, mfr: &DbManufacturer) {
        self.add_item::<DbManufacturer>(MFR_BUCKET, mfr);
    }

    fn get_part_id(&self, name: &str) -> Option<String> {
        self.get_item_id::<DbPart>(PARTS_BUCKET, name)
    }

    fn get_category_id(&self, name: &str) -> Option<String> {
        self.get_item_id::<DbCategory>(CATEGORIES_BUCKET, name)
    }

    fn get_package_id(&self, name: &str) -> Option<String> {
        self.get_item_id::<DbPackage>(PACKAGES_BUCKET, name)
    }

    fn get_mfr_id(&self, name: &str) -> Option<String> {
        self.get_item_id::<DbManufacturer>(MFR_BUCKET, name)
    }

    fn get_part_from_id(&self, id: &str) -> Option<DbPart> {
        self.get_item::<DbPart>(PARTS_BUCKET, id)
    }

    fn get_category_from_id(&self, id: &str) -> Option<DbCategory> {
        self.get_item::<DbCategory>(CATEGORIES_BUCKET, id)
    }

    fn get_package_from_id(&self, id: &str) -> Option<DbPackage> {
        self.get_item::<DbPackage>(PACKAGES_BUCKET, id)
    }

    fn get_mfr_from_id(&self, id: &str) -> Option<DbManufacturer> {
        self.get_item::<DbManufacturer>(MFR_BUCKET, id)
    }

    fn get_parts(&self) -> Vec<DbPart> {
        self.get_items::<DbPart>(PARTS_BUCKET)
    }

    fn get_categories(&self) -> Vec<DbCategory> {
        self.get_items::<DbCategory>(CATEGORIES_BUCKET)
    }

    fn get_packages(&self) -> Vec<DbPackage> {
        self.get_items::<DbPackage>(PACKAGES_BUCKET)
    }

    fn get_mfrs(&self) -> Vec<DbManufacturer> {
        self.get_items::<DbManufacturer>(MFR_BUCKET)
    }

    fn delete_part(&self, id: &str) -> String {
        self.delete_item(PARTS_BUCKET, id)
    }

    fn delete_category(&self, id: &str) -> String {
        self.delete_item(CATEGORIES_BUCKET, id)
    }

    fn delete_package(&self, id: &str) -> String {
        self.delete_item(PACKAGES_BUCKET, id)
    }

    fn delete_mfr(&self, id: &str) -> String {
        self.delete_item(MFR_BUCKET, id)
    }

    fn update_part(&self, name: &str, part: &DbPart) {
        let id = self.get_part_id(name);
        if id.is_none() {
            return;
        }

        let db = DB::open(self.path).unwrap();
        let tx = db.tx(true).unwrap();
        let bkt = tx.get_bucket(PARTS_BUCKET).unwrap();

        let value = rmp_serde::to_vec(&part).unwrap();
        bkt.put(id.unwrap(), value).unwrap();
        let _ = tx.commit();
    }
}
