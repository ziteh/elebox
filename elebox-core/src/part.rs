use crate::{comm::*, errors::*, jamm_db::*, json::*, yaml::*};

use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};

const DELETED_ITEM: &str = "__DELETE__";

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

pub struct PartHandler<'a> {
    pub(crate) db: &'a dyn Database<DbPart>,
    pub(crate) pkg_db: &'a dyn Database<DbPackage>,
    pub(crate) cat_db: &'a dyn Database<DbCategory>,
    pub(crate) mfr_db: &'a dyn Database<DbManufacturer>,
}

impl PartHandler<'_> {
    fn to_item(&self, db_part: DbPart) -> Result<Part, EleboxError> {
        let category = match self.cat_db.get(&db_part.category_id) {
            Ok(item) => item.name,
            Err(err) => match err {
                DbError::NotExists(_) => String::from(DELETED_ITEM),
                _ => return Err(EleboxError::DatabaseError(err)),
            },
        };

        let package = match self.pkg_db.get(&db_part.package_id) {
            Ok(item) => Some(item.name),
            Err(err) => match err {
                DbError::NotExists(_) => Some(String::from(DELETED_ITEM)),
                _ => return Err(EleboxError::DatabaseError(err)),
            },
        };

        let mfr = match self.mfr_db.get(&db_part.mfr_id) {
            Ok(item) => Some(item.name),
            Err(err) => match err {
                DbError::NotExists(_) => Some(String::from(DELETED_ITEM)),
                _ => return Err(EleboxError::DatabaseError(err)),
            },
        };

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

        Ok(part)
    }

    fn to_db_item(&self, item: &Part) -> Result<DbPart, EleboxError> {
        let category_id = match self.cat_db.get_id(&item.category) {
            Ok(id) => id,
            Err(err) => match err {
                DbError::NotExists(_) => String::from(DELETED_ITEM),
                _ => return Err(EleboxError::DatabaseError(err)),
            },
        };

        let package_id = match &item.package {
            Some(name) => match self.pkg_db.get_id(&name) {
                Ok(id) => id,
                Err(err) => match err {
                    DbError::NotExists(_) => String::from(DELETED_ITEM),
                    _ => return Err(EleboxError::DatabaseError(err)),
                },
            },
            None => "".to_string(),
        };

        let mfr_id = match &item.mfr {
            Some(name) => match self.mfr_db.get_id(&name) {
                Ok(id) => id,
                Err(err) => match err {
                    DbError::NotExists(_) => String::from(DELETED_ITEM),
                    _ => return Err(EleboxError::DatabaseError(err)),
                },
            },
            None => "".to_string(),
        };

        let unwrap_or_empty = |opt: &Option<String>| opt.as_deref().unwrap_or("").to_string();

        let db_part = DbPart {
            name: item.name.to_string(),
            quantity: item.quantity,
            category_id,
            mfr_id,
            package_id,
            package_detail: unwrap_or_empty(&item.package_detail),
            alias: unwrap_or_empty(&item.alias),
            description: unwrap_or_empty(&item.description),
            location: unwrap_or_empty(&item.location),
            mfr_no: unwrap_or_empty(&item.mfr_no),
            datasheet_link: unwrap_or_empty(&item.datasheet_link),
            product_link: unwrap_or_empty(&item.product_link),
            image_link: unwrap_or_empty(&item.image_link),
            custom_fields: item.custom_fields.clone(),
            suppliers: item.suppliers.clone(),
            starred: item.starred,
        };

        Ok(db_part)
    }

    pub fn update_part_quantity(&self, name: &str, increment: i16) -> Result<(), EleboxError> {
        let id = self.db.get_id(name)?;
        let mut db_item = self.db.get(id.as_str())?;

        let new_qty = db_item.quantity as i16 + increment;
        if new_qty < 0 {
            return Err(EleboxError::InventoryShortage(name.to_string()));
        } else {
            db_item.quantity = new_qty as u16;
        }

        self.db.update(&id, &db_item)?;
        Ok(())
    }
}

impl<'a> Handler<Part> for PartHandler<'_> {
    fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_id(name)?;
        let _ = self.db.delete(&id)?;
        Ok(())
    }

    fn add(&self, item: &Part) -> Result<(), EleboxError> {
        if self.db.get_id(&item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_PART),
                item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(item)?;
        let _ = self.db.add(&db_item)?;
        Ok(())
    }

    fn update(&self, ori_name: &str, new_item: &Part) -> Result<(), EleboxError> {
        let ori_id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_PART),
                new_item.name.clone(),
            ));
        }

        let db_part = self.to_db_item(new_item)?;
        let _ = self.db.update(ori_id.as_str(), &db_part)?;
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Part, EleboxError> {
        let id = self.db.get_id(name)?;
        let db_part = self.db.get(&id)?;
        self.to_item(db_part)
    }

    fn list(&self) -> Result<Vec<Part>, EleboxError> {
        let db_items = self.db.list()?;
        let mut items: Vec<Part> = vec![];
        for db_item in db_items {
            match self.to_item(db_item) {
                Ok(item) => items.push(item),
                Err(err) => return Err(err),
            }
        }
        Ok(items)
    }
}

impl Transferable for PartHandler<'_> {
    fn export(&self, filename: &PathBuf) -> Result<(), EleboxError> {
        let items = self.list()?;

        if YamlFile::check_extension(filename) {
            let _ = YamlFile::write(filename, items);
        } else if JsonFile::check_extension(filename) {
            let _ = JsonFile::write(filename, items);
        } else {
            todo!()
        }

        Ok(())
    }

    fn import(&self, filename: &PathBuf) -> Result<(), EleboxError> {
        let res_items: Result<Vec<Part>, ()> = if YamlFile::check_extension(filename) {
            YamlFile::read(filename)
        } else if JsonFile::check_extension(filename) {
            JsonFile::read(filename)
        } else {
            todo!()
        };

        if res_items.is_err() {
            todo!()
        }

        let parts: Vec<Part> = res_items.unwrap();
        for part in parts {
            if let Err(e) = self.add(&part) {
                match e {
                    EleboxError::AlreadyExists(_, _) => continue,
                    others => panic!("{}", others.to_string()),
                }
            }
        }

        Ok(())
    }
}
