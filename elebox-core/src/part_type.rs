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

use errors::*;

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

const PART_TYPES_BUCKET: &str = "part_types";

mod part_type {

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

    pub fn delete_part_type(db_path: &String, name: &String) -> Result<(), EleboxError> {
        todo!()
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
        // Part type name is unique
        if get_part_type_id(db_path, name).is_some() {
            return Err(EleboxError::PartAlreadyExists(name.to_string()));
        }

        // Get the ID of parent type
        let p_id = match parent {
            Some(p_name) => match get_part_type_id(db_path, &p_name) {
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
}
