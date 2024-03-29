use crate::{db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const ROOT_TYPE: &str = "root";

#[derive(Debug, Deserialize, Serialize)]
pub struct PartType {
    pub name: String,
    pub parent: Option<String>,
}

impl PartType {
    pub fn list(db_path: &str) -> Vec<PartType> {
        let db_part_types = get_db_part_types(db_path);
        let mut part_types: Vec<PartType> = Vec::new();

        for db_pt in db_part_types {
            let db_parent = get_db_part_type_from_id(db_path, &db_pt.parent_id);

            part_types.push(PartType {
                name: db_pt.name,
                parent: match db_parent {
                    Some(p) => Some(p.name),
                    None => None,
                },
            });
        }
        return part_types;
    }

    pub fn get(db_path: &str, name: &str) -> Result<PartType, EleboxError> {
        let id = match get_part_type_id(db_path, name) {
            Some(id) => id,
            None => return Err(EleboxError::NotExists(name.to_string())),
        };

        let db_pt = match get_db_part_type_from_id(db_path, &id) {
            Some(pt) => pt,
            None => return Err(EleboxError::NotExists(id.to_string())),
        };

        let part_type = PartType {
            name: db_pt.name.to_string(),
            parent: match get_db_part_type_from_id(db_path, &db_pt.parent_id) {
                Some(pt) => Some(pt.name),
                None => None,
            },
        };
        return Ok(part_type);
    }

    pub fn delete(&self, db_path: &str) -> Result<(), EleboxError> {
        todo!()
    }

    pub fn delete_by_name(db_path: &str, name: &str) -> Result<String, EleboxError> {
        let id = get_part_type_id(db_path, name);
        if id.is_none(){
            return Err(EleboxError::NotExists(name.to_string()));
        }

       let res = delete_db_part_types(db_path, &id.unwrap());
        return Ok(res);
    }

    pub fn update(
        db_path: &str,
        old_name: &str,
        new_name: Option<&str>,
        new_parent: Option<&str>,
    ) -> Result<(), EleboxError> {
        let id = get_part_type_id(db_path, old_name);
        if id.is_none() {
            return Err(EleboxError::NotExists(old_name.to_string()));
        }

        let old_db_pt = get_db_part_type_from_id(db_path, id.as_ref().unwrap()).unwrap();

        let p_id = match new_parent {
            Some(name) => match get_part_type_id(db_path, name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(name.to_string())),
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

        add_db_part_type(db_path, &db_part_type);
        return Ok(());
    }

    pub fn add(&self, db_path: &str) -> Result<(), EleboxError> {
        // Part type name is unique
        if get_part_type_id(db_path, &self.name).is_some() {
            return Err(EleboxError::AlreadyExists(self.name.to_string()));
        }

        // Get the ID of parent type
        let p_id = match &self.parent {
            Some(p_name) => match get_part_type_id(db_path, &p_name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(p_name.to_string())),
            },
            None => ROOT_TYPE.to_string(),
        };

        let db_part_type = DbPartType {
            name: self.name.to_string(),
            parent_id: p_id,
        };

        add_db_part_type(db_path, &db_part_type);
        return Ok(());
    }
}
