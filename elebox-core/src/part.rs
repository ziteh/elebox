use crate::{db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub name: String,
    pub catrgory: String,
    pub quantity: u16,
    pub package: Option<String>,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub cost: Option<f32>,
    pub location: Option<String>,
    pub mfr: Option<String>,
    pub mfr_part: Option<String>,
    pub mouser_part: Option<String>,
    pub digikey_part: Option<String>,
    pub datasheet_url: Option<String>,
    pub product_url: Option<String>,
    pub image_url: Option<String>,
    pub octopart_url: Option<String>,
    pub suppliers: Option<String>,
}

impl Part {
    pub fn new(name: &str, category: &str, quantity: u16) -> Part {
        Part {
            name: name.to_string(),
            catrgory: category.to_string(),
            quantity,
            package: None,
            alias: None,
            description: None,
            cost: None,
            location: None,
            mfr: None,
            mfr_part: None,
            mouser_part: None,
            digikey_part: None,
            datasheet_url: None,
            product_url: None,
            image_url: None,
            octopart_url: None,
            suppliers: None,
        }
    }

    pub fn delete_by_name(db_path: &str, name: &str) -> Result<(), EleboxError> {
        let id = get_part_id(db_path, name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        delete_db_part(db_path, &id.unwrap());
        return Ok(());
    }

    pub fn update(
        db_path: &str,
        old_name: &str,
        new_name: Option<&str>,
        new_quantity: Option<u16>,
        new_parent: Option<&str>,
    ) -> Result<(), EleboxError> {
        let id = get_part_id(db_path, old_name);
        if id.is_none() {
            return Err(EleboxError::NotExists(old_name.to_string()));
        }

        let old_db_part = get_db_part_from_id(db_path, id.as_ref().unwrap()).unwrap();

        let catrgory_id = match new_parent {
            Some(name) => match get_category_id(db_path, name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(name.to_string())),
            },
            None => old_db_part.category_id,
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
            category_id: catrgory_id,
        };

        add_db_part(db_path, &db_part);
        // {
        //     let db = DB::open(db_path).unwrap();
        //     let tx = db.tx(true).unwrap();
        //     let bucket = tx.get_bucket(PARTS_BUCKET).unwrap();

        //     let value = rmp_serde::to_vec(&db_part).unwrap();
        //     bucket.put(id.unwrap(), value).unwrap();
        //     let _ = tx.commit();
        // }

        return Ok(());
    }

    pub fn update_part_quantity(
        db_path: &String,
        name: &String,
        quantity: i16,
    ) -> Result<(), EleboxError> {
        let id = get_part_id(db_path, name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        let mut db_part = get_db_part_from_id(db_path, id.as_ref().unwrap()).unwrap();
        let new_q = db_part.quantity as i16 + quantity;
        if new_q < 0 {
            return Err(EleboxError::InventoryShortage(name.to_string()));
        } else {
            db_part.quantity = new_q as u16;
        }

        update_db_part(db_path, name, &db_part);
        return Ok(());
    }

    pub fn add(&self, db_path: &str) -> Result<(), EleboxError> {
        if get_part_id(db_path, &self.name).is_some() {
            return Err(EleboxError::AlreadyExists(self.name.to_string()));
        }

        let category_id = get_category_id(db_path, &self.catrgory);
        let db_part = DbPart {
            name: self.name.to_string(),
            quantity: self.quantity,
            category_id: match category_id {
                Some(id) => id.to_string(),
                None => "none".to_string(),
            },
        };

        add_db_part(db_path, &db_part);
        return Ok(());
    }

    pub fn list(db_path: &str) -> Vec<Part> {
        let db_parts = get_db_parts(db_path);
        let mut parts: Vec<Part> = Vec::new();

        for db_part in db_parts {
            let category = match get_db_category_from_id(db_path, &db_part.category_id) {
                Some(pt) => pt.name,
                None => "none".to_string(),
            };
            parts.push(Part::new(&db_part.name, &category, db_part.quantity));
        }
        return parts;
    }
}
