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
    pub fn new(name: &str, category: &str, quantity: u16) -> Self {
        Self {
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
}

pub struct PartManager<'a> {
    db: &'a dyn Datebase,
}

impl<'a> PartManager<'a> {
    pub fn new(db: &'a dyn Datebase) -> Self {
        Self { db }
    }

    pub fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_part_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        self.db.delete_part(&id.unwrap());
        return Ok(());
    }

    pub fn update(
        &self,
        old_name: &str,
        new_name: Option<&str>,
        new_quantity: Option<u16>,
        new_parent: Option<&str>,
    ) -> Result<(), EleboxError> {
        let id = self.db.get_part_id(old_name);
        if id.is_none() {
            return Err(EleboxError::NotExists(old_name.to_string()));
        }

        let old_db_part = self.db.get_part_from_id(id.as_ref().unwrap()).unwrap();

        let catrgory_id = match new_parent {
            Some(name) => match self.db.get_category_id(name) {
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

        self.db.add_part(&db_part);
        return Ok(());
    }

    pub fn update_part_quantity(&self, name: &String, quantity: i16) -> Result<(), EleboxError> {
        let id = self.db.get_part_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        let mut db_part = self.db.get_part_from_id(id.as_ref().unwrap()).unwrap();
        let new_q = db_part.quantity as i16 + quantity;
        if new_q < 0 {
            return Err(EleboxError::InventoryShortage(name.to_string()));
        } else {
            db_part.quantity = new_q as u16;
        }

        self.db.update_part(name, &db_part);
        return Ok(());
    }

    pub fn add(&self, part: &Part) -> Result<(), EleboxError> {
        if self.db.get_part_id(&part.name).is_some() {
            return Err(EleboxError::AlreadyExists(part.name.to_string()));
        }

        let category_id = self.db.get_category_id(&part.catrgory);
        let db_part = DbPart {
            name: part.name.to_string(),
            quantity: part.quantity,
            category_id: match category_id {
                Some(id) => id.to_string(),
                None => "none".to_string(),
            },
        };

        self.db.add_part(&db_part);
        return Ok(());
    }

    pub fn list(&self) -> Vec<Part> {
        let db_parts = self.db.get_parts();
        let mut parts: Vec<Part> = Vec::new();

        for db_part in db_parts {
            let category = match self.db.get_category_from_id(&db_part.category_id) {
                Some(pt) => pt.name,
                None => "none".to_string(),
            };
            parts.push(Part::new(&db_part.name, &category, db_part.quantity));
        }
        return parts;
    }
}
