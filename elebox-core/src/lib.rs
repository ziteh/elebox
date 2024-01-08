use core::fmt;
use std::{
    collections::{hash_map::RandomState, HashMap},
    error::Error,
    fmt::{Debug, Display},
    io::Read,
    ptr,
    str::{self, from_utf8},
};

use jammdb::{Data, KVPair, DB};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub enum EleboxError {
    PartAlreadyExists(String),
    PartNotExists(String),
    PartInventoryShortage(String),
}

impl Error for EleboxError {}

impl fmt::Display for EleboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EleboxError::PartAlreadyExists(ref name) => write!(f, "Part {} already exists", name),
            EleboxError::PartNotExists(ref name) => write!(f, "Part {} does not exists", name),
            EleboxError::PartInventoryShortage(ref name) => {
                write!(f, "Part {} not enough stock", name)
            }
        }
    }
}

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

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbPartType {
    pub name: String,
    pub parent_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartType {
    pub name: String,
    pub parent: String,
}

const DEFAULT_DATABASE_PATH: &str = "elebox.db";
const PARTS_BUCKET: &str = "parts";
const PART_TYPES_BUCKET: &str = "part_types";

pub fn init(db_path: &String) {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(true).unwrap();

    tx.get_or_create_bucket(PARTS_BUCKET).unwrap();
    tx.get_or_create_bucket(PART_TYPES_BUCKET).unwrap();
    let _ = tx.commit();
}

// ---------------------------------

pub fn get_part_type_id(db_path: &String, name: &String) -> Option<String> {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

    for data in bucket.cursor() {
        let part_type: DbPartType = rmp_serde::from_slice(data.kv().value()).unwrap();
        if &part_type.name == name {
            let id = from_utf8(data.kv().key()).unwrap();
            return Some(id.to_string());
        };
    }
    return None;
}

pub fn get_part_types(db_path: &String) -> Vec<PartType> {
    let mut part_types: Vec<PartType> = Vec::new();
    let mut db_part_types: Vec<DbPartType> = Vec::new();

    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(false).unwrap();
        let bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

        for data in bucket.cursor() {
            let part_type: DbPartType = rmp_serde::from_slice(data.kv().value()).unwrap();
            db_part_types.push(part_type);
        }
    }

    for db_pt in db_part_types {
        let db_parent = get_db_part_type_from_id(db_path, &db_pt.parent_id);
        part_types.push(PartType {
            name: db_pt.name,
            parent: match db_parent {
                Some(p) => p.name,
                None => "root".to_string(),
            },
        });
    }
    return part_types;
}

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

pub fn get_db_part_type_from_id(db_path: &String, id: &String) -> Option<DbPartType> {
    let db = DB::open(db_path).unwrap();
    let tx = db.tx(false).unwrap();
    let bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

    if let Some(data) = bucket.get(id) {
        let db_part_type: DbPartType = rmp_serde::from_slice(data.kv().value()).unwrap();
        return Some(db_part_type);
    }
    return None;
}

pub fn get_part_type_from_id(db_path: &String, id: &String) -> Option<PartType> {
    let db_part_type = match get_db_part_type_from_id(db_path, id) {
        Some(pt) => pt,
        None => return None,
    };

    let parent = match get_db_part_type_from_id(db_path, &db_part_type.parent_id) {
        Some(pt) => pt.name,
        None => "none".to_string(),
    };

    let part_type = PartType {
        name: db_part_type.name,
        parent,
    };
    return Some(part_type);
}

pub fn get_part_type(db_path: &String, name: &String) -> Option<PartType> {
    let id = match get_part_type_id(db_path, name) {
        Some(id) => id,
        None => return None,
    };

    let db_pt = match get_db_part_type_from_id(db_path, &id) {
        Some(pt) => pt,
        None => return None,
    };

    let part_type = PartType {
        name: db_pt.name.to_string(),
        parent: match get_db_part_type_from_id(db_path, &db_pt.parent_id) {
            Some(pt) => pt.name,
            None => "none".to_string(),
        },
    };
    return Some(part_type);
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

pub fn update_part_type(
    db_path: &String,
    old_name: &String,
    new_name: Option<&String>,
    new_parent: Option<&String>,
) -> Result<(), EleboxError> {
    let id = get_part_type_id(db_path, old_name);
    if id.is_none() {
        return Err(EleboxError::PartNotExists(old_name.to_string()));
    }

    let old_db_pt = get_db_part_type_from_id(db_path, id.as_ref().unwrap()).unwrap();

    let p_id = match new_parent {
        Some(name) => match get_part_type_id(db_path, name) {
            Some(id) => id,
            None => return Err(EleboxError::PartNotExists(name.to_string())),
        },
        None => old_db_pt.parent_id,
    };

    let db_part_type = DbPartType {
        name: match new_name {
            Some(name) => name.to_string(),
            None => old_db_pt.name,
        },
        parent_id: p_id,
    };

    {
        let db = DB::open(db_path).unwrap();
        let tx = db.tx(true).unwrap();
        let bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

        let value = rmp_serde::to_vec(&db_part_type).unwrap();
        bucket.put(id.unwrap(), value).unwrap();
        let _ = tx.commit();
    }

    Ok(())
}

pub fn add_part_type(
    db_path: &String,
    name: &String,
    parent: Option<&String>,
) -> Result<(), EleboxError> {
    if get_part_type_id(db_path, name).is_some() {
        return Err(EleboxError::PartAlreadyExists(name.to_string()));
    }

    let p_id = match parent {
        Some(name) => match get_part_type_id(db_path, name) {
            Some(id) => Some(id),
            None => None,
        },
        None => None,
    };

    let db_part_type = DbPartType {
        name: name.to_string(),
        parent_id: match p_id {
            Some(id) => id.to_string(),
            None => "root".to_string(),
        },
    };

    let db = DB::open(db_path).unwrap();
    let tx = db.tx(true).unwrap();
    let bucket = tx.get_bucket(PART_TYPES_BUCKET).unwrap();

    let value = rmp_serde::to_vec(&db_part_type).unwrap();
    let id = Uuid::new_v4().to_string();
    bucket.put(id, value).unwrap();
    let _ = tx.commit();

    Ok(())
}

// ---------------------------------

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
