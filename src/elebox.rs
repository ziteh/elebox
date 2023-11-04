use std::any::Any;

use jammdb::{Data, Error, KVPair, DB};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Part {
    pub part_number: String,
    pub quantity: u16,
    pub part_type_key: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PartType {
    pub name: String,
    pub parent_uuid: String,
}

const DEFAULT_DATABASE_PATH: &str = "elebox.db";
const PARTS_BUCKET: &str = "parts";
const PART_TYPES_BUCKET: &str = "part_types";

pub fn init() {
    let db = DB::open(DEFAULT_DATABASE_PATH).unwrap();
    let tx = db.tx(true).unwrap();

    tx.create_bucket(PARTS_BUCKET).unwrap();
    tx.create_bucket(PART_TYPES_BUCKET).unwrap();
    tx.commit();
}

pub fn add_part(part: Part) {
    let db = DB::open(DEFAULT_DATABASE_PATH).unwrap();
    let tx = db.tx(true).unwrap();

    let parts_bucket = tx.get_bucket(PARTS_BUCKET).unwrap();
    let part_bucket = rmp_serde::to_vec(&part).unwrap();
    let id = Uuid::new_v4();

    parts_bucket.put(id.to_string(), part_bucket).unwrap();
    tx.commit();
}

pub fn add_part_type(p_type: PartType) {
    let db = DB::open(DEFAULT_DATABASE_PATH).unwrap();
    let tx = db.tx(true).unwrap();

    let types_bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();
    let type_bucket = rmp_serde::to_vec(&p_type).unwrap();
    let id = Uuid::new_v4();

    types_bucket.put(id.to_string(), type_bucket).unwrap();
    tx.commit();
}

pub fn read_part_types() -> Vec<PartType> {
    let mut types: Vec<PartType> = Vec::new();

    let db = DB::open(DEFAULT_DATABASE_PATH).unwrap();
    let tx = db.tx(true).unwrap();
    let types_bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

    for data in types_bucket.cursor() {
        let p = rmp_serde::from_slice(data.kv().value()).unwrap();
        types.push(p);
    }
    return types;
}

pub fn read() {
    let db = DB::open("my.db").unwrap();
    let tx = db.tx(false).unwrap();

    let parts_bucket = tx.get_bucket("parts").unwrap();

    if let Some(data) = parts_bucket.get("p1") {
        if data.is_kv() {
            let kv = data.key();

            let p: Part = rmp_serde::from_slice(data.kv().value()).unwrap();
            print!("{}", p.part_number.to_string())
        }
    }
}
