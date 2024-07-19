use crate::{csv::*, db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::fmt::{format, Debug};

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
    pub starred: bool,
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
            starred: false,
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

    pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
        let id = self
            .db
            .get_part_id(name)
            .ok_or(EleboxError::NotExists("Part".to_string(), name.to_string()))?;

        return Ok(self.db.delete_part(&id));
    }

    fn to_db_part(&self, part: &Part) -> Result<DbPart, EleboxError> {
        if self.db.get_part_id(&part.name).is_some() {
            return Err(EleboxError::AlreadyExists(
                "Part".to_string(),
                part.name.clone(),
            ));
        }

        let category_id = match self.db.get_category_id(&part.category) {
            Some(id) => id.to_string(),
            None => {
                return Err(EleboxError::NotExists(
                    "Category".to_string(),
                    part.category.clone(),
                ))
            }
        };

        let package_id = match &part.package {
            Some(name) => match self.db.get_package_id(&name) {
                Some(id) => id,
                None => {
                    return Err(EleboxError::NotExists(
                        "Package".to_string(),
                        name.to_string(),
                    ))
                }
            },
            None => "".to_string(),
        };

        let mfr_id = match &part.mfr {
            Some(name) => match self.db.get_mfr_id(&name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists("Mfr".to_string(), name.to_string())),
            },
            None => "".to_string(),
        };

        let unwrap_or_empty = |opt: &Option<String>| opt.as_deref().unwrap_or("").to_string();

        let db_part = DbPart {
            name: part.name.to_string(),
            quantity: part.quantity,
            category_id,
            mfr_id,
            package_id,
            package_detail: unwrap_or_empty(&part.package_detail),
            alias: unwrap_or_empty(&part.alias),
            description: unwrap_or_empty(&part.description),
            location: unwrap_or_empty(&part.location),
            mfr_no: unwrap_or_empty(&part.mfr_no),
            datasheet_link: unwrap_or_empty(&part.datasheet_link),
            product_link: unwrap_or_empty(&part.product_link),
            image_link: unwrap_or_empty(&part.image_link),
            custom_fields: part.custom_fields.clone(),
            suppliers: part.suppliers.clone(),
            starred: part.starred,
        };

        Ok(db_part)
    }

    pub fn update(&self, ori_name: &str, new_part: &Part) -> Result<(), EleboxError> {
        if self.db.get_part_id(ori_name).is_none() {
            return Err(EleboxError::NotExists(
                "Origin part".to_string(),
                ori_name.to_string(),
            ));
        }

        let db_part = self.to_db_part(new_part)?;
        self.db.update_part(ori_name, &db_part);
        Ok(())
    }

    pub fn update_part_quantity(&self, name: &str, increment: i16) -> Result<(), EleboxError> {
        let id = self.db.get_part_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists("Part".to_string(), name.to_string()));
        }

        let mut db_part = self.db.get_part_from_id(id.as_ref().unwrap()).unwrap();
        let new_q = db_part.quantity as i16 + increment;
        if new_q < 0 {
            return Err(EleboxError::InventoryShortage(name.to_string()));
        } else {
            db_part.quantity = new_q as u16;
        }

        self.db.update_part(name, &db_part);
        return Ok(());
    }

    pub fn add(&self, part: &Part) -> Result<(), EleboxError> {
        let db_part = self.to_db_part(part)?;
        self.db.add_part(&db_part);
        Ok(())
    }

    fn from_db_part(&self, db_part: DbPart) -> Result<Part, EleboxError> {
        let category = self
            .db
            .get_category_from_id(&db_part.category_id)
            .ok_or(EleboxError::NotExists(
                "Category".to_string(),
                db_part.category_id,
            ))?
            .name;

        let package = self
            .db
            .get_package_from_id(&db_part.package_id)
            .map(|pkg| pkg.name);

        let mfr = self.db.get_mfr_from_id(&db_part.mfr_id).map(|mfr| mfr.name);

        let part = Part {
            name: db_part.name,
            category,
            quantity: db_part.quantity,
            package,
            package_detail: Some(db_part.package_detail),
            mfr,
            alias: Some(db_part.alias),
            description: Some(db_part.description),
            location: Some(db_part.location),
            mfr_no: Some(db_part.mfr_no),
            datasheet_link: Some(db_part.datasheet_link),
            product_link: Some(db_part.product_link),
            image_link: Some(db_part.image_link),
            custom_fields: db_part.custom_fields,
            suppliers: db_part.suppliers,
            starred: db_part.starred,
        };

        return Ok(part);
    }

    pub fn get(&self, name: &str) -> Result<Part, EleboxError> {
        let id = self
            .db
            .get_part_id(name)
            .ok_or(EleboxError::NotExists("Part".to_string(), name.to_string()))?;

        let db_part = self
            .db
            .get_part_from_id(&id)
            .ok_or(EleboxError::NotExists("Part".to_string(), id))?;

        self.from_db_part(db_part)
    }

    pub fn list(&self) -> Vec<Part> {
        self.db
            .get_parts()
            .into_iter()
            .filter_map(|db_p| self.from_db_part(db_p).ok())
            .collect()
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
