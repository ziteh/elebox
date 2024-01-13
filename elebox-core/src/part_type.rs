mod part_type {

    use crate::{db::*, errors::EleboxError};
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PartType {
        pub name: String,
        pub parent: String,
    }

    const ROOT_TYPE: &str = "root";

    pub fn get_part_types(db_path: &String) -> Vec<PartType> {
        let mut part_types: Vec<PartType> = Vec::new();
        let db_part_types = get_db_part_types(db_path);

        for db_pt in db_part_types {
            let db_parent = get_db_part_type_from_id(db_path, &db_pt.parent_id);

            part_types.push(PartType {
                name: db_pt.name,
                parent: match db_parent {
                    Some(p) => p.name,
                    None => ROOT_TYPE.to_string(),
                },
            });
        }
        return part_types;
    }

    pub fn get_part_type(db_path: &str, name: &str) -> Result<PartType, EleboxError> {
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
                Some(pt) => pt.name,
                None => ROOT_TYPE.to_string(),
            },
        };
        return Ok(part_type);
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

    pub fn add_part_type(
        db_path: &str,
        name: &str,
        parent: Option<&str>,
    ) -> Result<(), EleboxError> {
        // Part type name is unique
        if get_part_type_id(db_path, name).is_some() {
            return Err(EleboxError::AlreadyExists(name.to_string()));
        }

        // Get the ID of parent type
        let p_id = match parent {
            Some(p_name) => match get_part_type_id(db_path, &p_name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(p_name.to_string())),
            },
            None => ROOT_TYPE.to_string(),
        };

        let db_part_type = DbPartType {
            name: name.to_string(),
            parent_id: p_id,
        };

        add_db_part_type(db_path, &db_part_type);
        return Ok(());
    }
}
