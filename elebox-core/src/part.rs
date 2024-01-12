mod errors;

use std::{
    collections::{hash_map::RandomState, HashMap},
    fmt::Debug,
    io::Read,
    ptr,
    str::{self, from_utf8},
};

use jammdb::{Data, KVPair, DB};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use errors::*;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbPart {
    pub name: String,
    pub type_id: String,
    pub quantity: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub name: String,
    pub part_type: String,
    pub quantity: u16,
}

const PARTS_BUCKET: &str = "parts";

pub fn get_db_part_from_id(db_path: &String, id: &String) -> Option<DbPart> {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

    if let Some(data) = bucket.get(id) {
        let db_part: DbPart = rmp_serde::from_slice(data.kv().value()).unwrap();
        return Some(db_part);
    }
    return None;
}

pub fn delete_part(db_path: &String, name: &String) -> Result<(), EleboxError> {
    let id = get_part_id(db_path, name);
    if id.is_none() {
        return Err(EleboxError::PartNotExists(name.to_string()));
    }

    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(true).unwrap();
        let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

        // assert!(bucket.get_kv(&id.unwrap()).is_some());
        let _ = bucket.delete(&id.unwrap());
        let _ = tx.commit();
        // assert!(bucket.get_kv(id.unwrap()).is_none());
    }

    Ok(())
}

pub fn update_part(
    db_path: &String,
    old_name: &String,
    new_name: Option<&String>,
    new_quantity: Option<u16>,
    new_parent: Option<&String>,
) -> Result<(), EleboxError> {
    let id = get_part_id(db_path, old_name);
    if id.is_none() {
        return Err(EleboxError::PartNotExists(old_name.to_string()));
    }

    let old_db_part = get_db_part_from_id(db_path, id.as_ref().unwrap()).unwrap();

    let type_id = match new_parent {
        Some(name) => match get_part_type_id(db_path, name) {
            Some(id) => id,
            None => return Err(EleboxError::PartNotExists(name.to_string())),
        },
        None => old_db_part.type_id,
    };

    let db_part = DbPart {
        name: match new_name {
            Some(name) => name.to_string(),
            None => old_db_part.name,
        },
        quantity: match new_quantity {
            Some(q) => q,
            None => old_db_part.quantity,
        },
        type_id,
    };

    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(true).unwrap();
        let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

        let value = rmp_serde::to_vec(&db_part).unwrap();
        bucket.put(id.unwrap(), value).unwrap();
        let _ = tx.commit();
    }

    Ok(())
}

pub fn update_part_quantity(
    db_path: &String,
    name: &String,
    quantity: i16,
) -> Result<(), EleboxError> {
    let id = get_part_id(db_path, name);
    if id.is_none() {
        return Err(EleboxError::PartNotExists(name.to_string()));
    }

    let mut db_part = get_db_part_from_id(db_path, id.as_ref().unwrap()).unwrap();
    let new_q = db_part.quantity as i16 + quantity;
    if new_q < 0 {
        return Err(EleboxError::PartInventoryShortage(name.to_string()));
    } else {
        db_part.quantity = new_q as u16;
    }

    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(true).unwrap();
        let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

        let value = rmp_serde::to_vec(&db_part).unwrap();
        bucket.put(id.unwrap(), value).unwrap();
        let _ = tx.commit();
    }

    Ok(())
}

pub fn get_part_id(db_path: &String, name: &String) -> Option<String> {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

    for data in bucket.cursor() {
        let part: DbPart = rmp_serde::from_slice(data.kv().value()).unwrap();
        if &part.name == name {
            let id = from_utf8(data.kv().key()).unwrap();
            return Some(id.to_string());
        };
    }
    return None;
}

pub fn add_part(
    db_path: &String,
    name: &String,
    quantity: &u16,
    part_type: &String,
) -> Result<(), EleboxError> {
    if get_part_id(db_path, name).is_some() {
        return Err(EleboxError::PartAlreadyExists(name.to_string()));
    }

    let part_type_id = get_part_type_id(db_path, part_type);
    let db_part = DbPart {
        name: name.to_string(),
        quantity: *quantity,
        type_id: match part_type_id {
            Some(id) => id.to_string(),
            None => "none".to_string(),
        },
    };

    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(true).unwrap();
        let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

        let value = rmp_serde::to_vec(&db_part).unwrap();
        let id = Uuid::new_v4().to_string();
        bucket.put(id, value).unwrap();
        let _ = tx.commit();
    }

    Ok(())
}

pub fn get_parts(db_path: &String) -> Vec<Part> {
    let mut db_parts: Vec<DbPart> = Vec::new();
    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(false).unwrap();
        let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

        for data in bucket.cursor() {
            let db_part: DbPart = rmp_serde::from_slice(data.kv().value()).unwrap();
            db_parts.push(db_part);
        }
    }

    let mut parts: Vec<Part> = Vec::new();
    for db_part in db_parts {
        parts.push(Part {
            name: db_part.name,
            quantity: db_part.quantity,
            part_type: match get_part_type_from_id(db_path, &db_part.type_id) {
                Some(pt) => pt.name,
                None => "none".to_string(),
            },
        })
    }
    return parts;
}
