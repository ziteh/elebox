use crate::{csv::*, db::*, errors::EleboxError, TreeNode};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Part {
    pub name: String,
    pub quantity: u16,
    pub category: String,
    pub package: Option<String>,
    pub package_detail: Option<String>,
    pub alias: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub mfr: Option<String>,
    pub mfr_no: Option<String>,
    pub datasheet_link: Option<String>,
    pub product_link: Option<String>,
    pub image_link: Option<String>,
    pub custom_fields: Vec<CustomField>,
    pub suppliers: Vec<Supplier>,
}

impl Part {
    pub fn new(name: &str, category: &str, quantity: u16) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            quantity,
            package: None,
            package_detail: None,
            mfr: None,
            alias: None,
            description: None,
            location: None,
            mfr_no: None,
            datasheet_link: None,
            product_link: None,
            image_link: None,
            custom_fields: vec![],
            suppliers: vec![],
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

    pub fn update(&self, name: &str, new_part: &Part) -> Result<(), EleboxError> {
        let id = self.db.get_part_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        let category_id = match self.db.get_category_id(&new_part.category) {
            Some(id) => id.to_string(),
            None => "none".to_string(), // TODO: empty value
        };

        let package_id = match &new_part.package {
            Some(n) => match self.db.get_package_id(&n) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(n.to_string())),
            },
            None => "".to_string(),
        };

        let mfr_id = match &new_part.mfr {
            Some(n) => match self.db.get_mfr_id(&n) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(n.to_string())),
            },
            None => "".to_string(),
        };

        let empty = &"".to_string();

        let alias = new_part.alias.as_ref().unwrap_or(empty);
        let package_detail = new_part.package_detail.as_ref().unwrap_or(empty);
        let description = new_part.description.as_ref().unwrap_or(empty);
        let location = new_part.location.as_ref().unwrap_or(empty);
        let mfr_no = new_part.mfr_no.as_ref().unwrap_or(empty);
        let datasheet_link = new_part.datasheet_link.as_ref().unwrap_or(empty);
        let product_link = new_part.product_link.as_ref().unwrap_or(empty);
        let image_link = new_part.image_link.as_ref().unwrap_or(empty);

        let db_part = DbPart {
            name: new_part.name.to_string(),
            quantity: new_part.quantity,
            category_id,
            package_id,
            package_detail: package_detail.to_string(),
            mfr_id,
            alias: alias.to_string(),
            description: description.to_string(),
            location: location.to_string(),
            mfr_no: mfr_no.to_string(),
            datasheet_link: datasheet_link.to_string(),
            product_link: product_link.to_string(),
            image_link: image_link.to_string(),
            custom_fields: new_part.custom_fields.clone(),
            suppliers: new_part.suppliers.clone(),
        };

        self.db.update_part(name, &db_part);
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
            None => "none".to_string(), // TODO: empty value
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
        let package_detail = part.package_detail.as_ref().unwrap_or(empty);
        let description = part.description.as_ref().unwrap_or(empty);
        let location = part.location.as_ref().unwrap_or(empty);
        let mfr_no = part.mfr_no.as_ref().unwrap_or(empty);
        let datasheet_link = part.datasheet_link.as_ref().unwrap_or(empty);
        let product_link = part.product_link.as_ref().unwrap_or(empty);
        let image_link = part.image_link.as_ref().unwrap_or(empty);

        let db_part = DbPart {
            name: part.name.to_string(),
            quantity: part.quantity,
            category_id,
            package_id,
            package_detail: package_detail.to_string(),
            mfr_id,
            alias: alias.to_string(),
            description: description.to_string(),
            location: location.to_string(),
            mfr_no: mfr_no.to_string(),
            datasheet_link: datasheet_link.to_string(),
            product_link: product_link.to_string(),
            image_link: image_link.to_string(),
            custom_fields: part.custom_fields.clone(),
            suppliers: part.suppliers.clone(),
        };

        self.db.add_part(&db_part);
        return Ok(());
    }

    pub fn get(&self, name: &str) -> Result<Part, EleboxError> {
        let id = self.db.get_part_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        let db_part = self.db.get_part_from_id(&id.unwrap()).unwrap();
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
        part.package_detail = Some(db_part.package_detail);
        part.mfr = mfr;
        part.alias = Some(db_part.alias);
        part.description = Some(db_part.description);
        part.location = Some(db_part.location);
        part.mfr_no = Some(db_part.mfr_no);
        part.datasheet_link = Some(db_part.datasheet_link);
        part.product_link = Some(db_part.product_link);
        part.image_link = Some(db_part.image_link);
        part.custom_fields = db_part.custom_fields;
        part.suppliers = db_part.suppliers;

        return Ok(part);
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
            part.package_detail = Some(db_part.package_detail);
            part.alias = Some(db_part.alias);
            part.description = Some(db_part.description);
            part.location = Some(db_part.location);
            part.mfr_no = Some(db_part.mfr_no);
            part.datasheet_link = Some(db_part.datasheet_link);
            part.product_link = Some(db_part.product_link);
            part.image_link = Some(db_part.image_link);
            part.custom_fields = db_part.custom_fields;
            part.suppliers = db_part.suppliers;

            parts.push(part);
        }
        return parts;
    }

    pub fn export_csv(&self, filename: &str) -> Result<(), ()> {
        let parts = self.list();
        let res = write_csv(filename, parts, None);
        return res;
    }

    pub fn import_csv(&self, filename: &str) -> Result<(), ()> {
        let res_parts = read_csv(filename, None);
        if res_parts.is_err() {
            return Err(());
        }

        let parts: Vec<Part> = res_parts.unwrap();
        for part in parts {
            let _ = self.add(&part);
        }

        Ok(())
    }
}
