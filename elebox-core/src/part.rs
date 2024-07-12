use crate::{db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub name: String,
    pub quantity: u16,
    pub category: String,
    pub package: Option<String>,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub cost: Option<f32>,
    pub location: Option<String>,
    pub mfr: Option<String>,
    pub mfr_no: Option<String>,
    pub mouser_no: Option<String>,
    pub digikey_no: Option<String>,
    pub datasheet_url: Option<String>,
    pub product_url: Option<String>,
    pub image_url: Option<String>,
    pub suppliers: Option<String>,
}

impl Part {
    pub fn new(name: &str, category: &str, quantity: u16) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            quantity,
            package: None,
            mfr: None,
            alias: None,
            description: None,
            cost: None,
            location: None,
            mfr_no: None,
            mouser_no: None,
            digikey_no: None,
            datasheet_url: None,
            product_url: None,
            image_url: None,
            suppliers: None,
        }
    }
}

pub struct PartManager<'a> {
    db: &'a dyn Database,
}

impl<'a> PartManager<'a> {
    pub fn new(db: &'a dyn Database) -> Self {
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
        new_category: Option<&str>,
    ) -> Result<(), EleboxError> {
        let id = self.db.get_part_id(old_name);
        if id.is_none() {
            return Err(EleboxError::NotExists(old_name.to_string()));
        }

        let mut db_part = self.db.get_part_from_id(id.as_ref().unwrap()).unwrap();

        let category_id = match new_category {
            Some(name) => match self.db.get_category_id(name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(name.to_string())),
            },
            None => db_part.category_id,
        };

        if new_name.is_some() {
            db_part.name = new_name.unwrap().to_string();
        }

        if new_quantity.is_some() {
            db_part.quantity = new_quantity.unwrap();
        }

        db_part.category_id = category_id;

        self.db.add_part(&db_part);
        return Ok(());
    }

    pub fn update_part_quantity(&self, name: &str, quantity: i16) -> Result<(), EleboxError> {
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

        let category_id = match self.db.get_category_id(&part.category) {
            Some(id) => id.to_string(),
            None => "none".to_string(),
        };

        let package_id = match &part.package {
            Some(n) => match self.db.get_package_id(&n) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(n.to_string())),
            },
            None => "".to_string(),
        };

        let mfr_id = match &part.mfr {
            Some(n) => match self.db.get_mfr_id(&n) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(n.to_string())),
            },
            None => "".to_string(),
        };

        let empty = &"".to_string();

        let alias = part.alias.as_ref().unwrap_or(empty);
        let description = part.description.as_ref().unwrap_or(empty);
        let location = part.location.as_ref().unwrap_or(empty);
        let mfr_no = part.mfr_no.as_ref().unwrap_or(empty);
        let mouser_no = part.mouser_no.as_ref().unwrap_or(empty);
        let digikey_no = part.digikey_no.as_ref().unwrap_or(empty);
        let datasheet_url = part.datasheet_url.as_ref().unwrap_or(empty);
        let product_url = part.product_url.as_ref().unwrap_or(empty);
        let image_url = part.image_url.as_ref().unwrap_or(empty);
        let suppliers = part.suppliers.as_ref().unwrap_or(empty);
        let cost = part.cost.as_ref().unwrap_or(&f32::NAN);

        let db_part = DbPart {
            name: part.name.to_string(),
            quantity: part.quantity,
            category_id,
            package_id,
            mfr_id,
            alias: alias.to_string(),
            description: description.to_string(),
            cost: *cost,
            location: location.to_string(),
            mfr_no: mfr_no.to_string(),
            mouser_no: mouser_no.to_string(),
            digikey_no: digikey_no.to_string(),
            datasheet_url: datasheet_url.to_string(),
            product_url: product_url.to_string(),
            image_url: image_url.to_string(),
            suppliers: suppliers.to_string(),
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

            let package = self
                .db
                .get_package_from_id(&db_part.package_id)
                .map(|p| p.name);
            let mfr = self.db.get_mfr_from_id(&db_part.mfr_id).map(|p| p.name);

            let mut part = Part::new(&db_part.name, &category, db_part.quantity);
            part.package = package;
            part.mfr = mfr;

            // TODO None if empty
            part.alias = Some(db_part.alias);
            part.description = Some(db_part.description);
            part.location = Some(db_part.location);
            part.mfr_no = Some(db_part.mfr_no);
            part.mouser_no = Some(db_part.mouser_no);
            part.digikey_no = Some(db_part.digikey_no);
            part.datasheet_url = Some(db_part.datasheet_url);
            part.product_url = Some(db_part.product_url);
            part.image_url = Some(db_part.image_url);
            part.suppliers = Some(db_part.suppliers);
            part.cost = Some(db_part.cost);

            parts.push(part);
        }
        return parts;
    }
}
