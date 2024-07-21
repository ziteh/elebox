use crate::{comm::*, jamm_db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub struct Manufacturer {
    pub name: String,
    pub alias: Option<String>,
    pub url: Option<String>,
}

impl Manufacturer {
    pub fn new(name: &str, alias: Option<&str>, url: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
        }
    }
}

pub struct ManufacturerManager {
    db: Box<dyn Database<DbManufacturer>>,
}

impl ManufacturerManager {
    pub fn new(path: &str) -> Self {
        Self {
            db: Box::new(JammDatabase::new(path, MFR_BUCKET)),
        }
    }

    fn to_db_item(&self, item: &Manufacturer) -> Result<DbManufacturer, EleboxError> {
        let db_mfr = DbManufacturer {
            name: item.name.to_string(),
            alias: match &item.alias {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            url: match &item.url {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        };
        Ok(db_mfr)
    }

    fn to_item(&self, db_item: &DbManufacturer) -> Manufacturer {
        Manufacturer::new(
            &db_item.name,
            Some(db_item.alias.as_str()).filter(|s| !s.is_empty()),
            Some(db_item.url.as_str()).filter(|s| !s.is_empty()),
        )
    }
}

impl Manager<Manufacturer> for ManufacturerManager {
    fn init(&self) -> Result<(), EleboxError> {
        let _ = self.db.init()?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_id(name)?;
        let _ = self.db.delete(&id)?;
        Ok(())
    }

    fn add(&self, item: &Manufacturer) -> Result<(), EleboxError> {
        if self.db.get_id(&item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_MFR),
                item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(item)?;
        let _ = self.db.add(&db_item)?;
        Ok(())
    }

    fn update(&self, ori_name: &str, new_item: &Manufacturer) -> Result<(), EleboxError> {
        let _id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_MFR),
                new_item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(new_item)?;
        let _ = self.db.update(ori_name, &db_item)?;
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Manufacturer, EleboxError> {
        let id = self.db.get_id(name)?;
        let db_item = self.db.get(&id)?;
        Ok(self.to_item(&db_item))
    }

    fn list(&self) -> Result<Vec<Manufacturer>, EleboxError> {
        let db_items = self.db.list()?;
        let mut items: Vec<Manufacturer> = vec![];
        for db_item in db_items {
            items.push(self.to_item(&db_item));
        }
        Ok(items)
    }
}
