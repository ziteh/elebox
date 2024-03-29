use std::{
    fmt::Debug,
    str::{self, from_utf8},
};

use jammdb::{Data, KVPair, DB};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{EleboxError, PartType};

trait DbItem {
    fn get_name(&self) -> String;
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbPart {
    pub name: String,
    pub type_id: String,
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
    pub part_type: String,
    pub quantity: u16,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbPartType {
    pub name: String,
    pub parent_id: String,
}

impl DbItem for DbPartType {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

const DEFAULT_DATABASE_PATH: &str = "./elebox.db";
const PARTS_BUCKET: &str = "parts";
const PART_TYPES_BUCKET: &str = "part_types";

pub fn init_db(db_path: &str) {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(true).unwrap();

    tx.get_or_create_bucket(PARTS_BUCKET).unwrap();
    tx.get_or_create_bucket(PART_TYPES_BUCKET).unwrap();
    let _ = tx.commit();
}

fn add_db_item<T>(db_path: &str, bucket: &str, item: &T)
where
    T: Serialize,
{
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(true).unwrap();
    let bkt = tx.get_bucket(bucket).unwrap();

    let value = rmp_serde::to_vec(&item).unwrap();
    let id = Uuid::new_v4().to_string();
    bkt.put(id, value).unwrap();
    let _ = tx.commit();
}

pub fn update_db_part(db_path: &str, name: &str, db_part: &DbPart) {
    let id = get_part_id(db_path, name);
    if id.is_none() {
        // return Err(EleboxError::NotExists(name.to_string()));
    }

    let db = DB::open(db_path).unwrap();
    let tx = db.tx(true).unwrap();
    let bkt = tx.get_bucket(PARTS_BUCKET).unwrap();

    let value = rmp_serde::to_vec(&db_part).unwrap();
    bkt.put(id.unwrap(), value).unwrap();
    let _ = tx.commit();
}

pub fn add_db_part(db_path: &str, db_part: &DbPart) {
    add_db_item::<DbPart>(db_path, PARTS_BUCKET, db_part);
}

pub fn add_db_part_type(db_path: &str, db_part_type: &DbPartType) {
    add_db_item::<DbPartType>(db_path, PART_TYPES_BUCKET, db_part_type);
}

fn get_item_id<T>(db_path: &str, bucket: &str, name: &str) -> Option<String>
where
    T: for<'a> Deserialize<'a> + DbItem,
{
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bkt = tx.get_bucket(bucket).unwrap();

    for data in bkt.cursor() {
        let part: T = rmp_serde::from_slice(data.kv().value()).unwrap();

        if &part.get_name() == name {
            let id = from_utf8(data.kv().key()).unwrap();
            return Some(id.to_string());
        };
    }
    return None;
}

pub fn get_part_id(db_path: &str, name: &str) -> Option<String> {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bkt = tx.get_bucket(PARTS_BUCKET).unwrap();

    for data in bkt.cursor() {
        let part: DbPart = rmp_serde::from_slice(data.kv().value()).unwrap();

        if &part.name == name {
            let id = from_utf8(data.kv().key()).unwrap();
            return Some(id.to_string());
        };
    }
    return None;
}

pub fn get_part_type_id(db_path: &str, name: &str) -> Option<String> {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bkt = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

    for data in bkt.cursor() {
        let part_type: DbPartType = rmp_serde::from_slice(data.kv().value()).unwrap();

        if &part_type.name == name {
            let id = from_utf8(data.kv().key()).unwrap();
            return Some(id.to_string());
        };
    }
    return None;
}

pub fn delete_db_part(db_path: &str, id: &str) {
    // let db = DB::open(db_path).unwrap();
    // let tx = db.tx(true).unwrap();
    // let bkt = tx.get_bucket(PARTS_BUCKET).unwrap();

    // // assert!(bkt.get_kv(id).is_some());

    // let _ = bkt.delete(id);
    // let _ = tx.commit();

    // // assert!(bkt.get_kv(id).is_none());

    delete_db_items(db_path, PARTS_BUCKET, id);
}

pub fn delete_db_part_types(db_path: &str, id: &str) ->String {
    delete_db_items(db_path, PART_TYPES_BUCKET, id)
}

pub fn delete_db_items(db_path: &str, bucket: &str, id:&str) -> String 
    {
       let db = DB::open(db_path).unwrap();
       let tx = db.tx(true).unwrap();
       let bkt = tx.get_bucket(bucket).unwrap(); 

    // assert!(bkt.get_kv(id).is_some());
       
       let res = bkt.delete(id);
    //    let _ = tx.commit();
       match res {
           Err(e)=> return e.to_string(),
           _=> return "ok".to_string()
       }

    // assert!(bkt.get_kv(id).is_none());
    }

pub fn get_db_items<T>(db_path: &str, bucket: &str) -> Vec<T>
where
    T: for<'a> Deserialize<'a>,
{
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bkt = tx.get_bucket(bucket).unwrap();

    let mut items: Vec<T> = Vec::new();
    for data in bkt.cursor() {
        let item: T = rmp_serde::from_slice(data.kv().value()).unwrap();
        items.push(item);
    }

    return items;
}

pub fn get_db_parts(db_path: &str) -> Vec<DbPart> {
    get_db_items::<DbPart>(db_path, PARTS_BUCKET)
}

pub fn get_db_part_types(db_path: &str) -> Vec<DbPartType> {
    get_db_items::<DbPartType>(db_path, PART_TYPES_BUCKET)
}

pub fn get_db_part_from_id(db_path: &str, id: &str) -> Option<DbPart> {
    get_db_item::<DbPart>(db_path, PARTS_BUCKET, id)
}

pub fn get_db_part_type_from_id(db_path: &str, id: &str) -> Option<DbPartType> {
    get_db_item::<DbPartType>(db_path, PART_TYPES_BUCKET, id)
}

fn get_db_item<T>(db_path: &str, bucket: &str, id: &str) -> Option<T>
where
    T: for<'a> Deserialize<'a>,
{
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bkt = tx.get_bucket(bucket).unwrap();

    if let Some(data) = bkt.get(id) {
        return Some(rmp_serde::from_slice(data.kv().value()).unwrap());
    }
    return None;
}

// pub fn get_part_type_from_id(db_path: &str, id: &str) -> Option<PartType> {
//     let db_part_type = match get_db_part_type_from_id(db_path, id) {
//         Some(pt) => pt,
//         None => return None,
//     };

//     let parent_name = match get_db_part_type_from_id(db_path, &db_part_type.parent_id) {
//         Some(pt) => pt.name,
//         None => "none".to_string(),
//     };

//     let part_type = PartType {
//         name: db_part_type.name,
//         parent: parent_name,
//     };
//     return Some(part_type);
// }
