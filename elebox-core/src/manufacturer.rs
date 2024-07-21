use crate::{comm::*, csv::*, db::*, errors::EleboxError, yaml::*};
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
    db: Box<dyn BaseDatabase<DbManufacturer>>,
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
        let id = self.db.get_id(ori_name)?;

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

impl ManufacturerManager {
    pub fn new(path: &str) -> Self {
        Self {
            db: Box::new(JammDatabase::new(path, MFR_BUCKET)),
        }
    }

    // pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
    //     let id = self
    //         .db
    //         .get_mfr_id(name)
    //         .ok_or(EleboxError::NotExists("Mfr".to_string(), name.to_string()))?;

    //     Ok(self.db.delete_mfr(&id))
    // }

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

    // pub fn add(&self, mfr: &Manufacturer) -> Result<(), EleboxError> {
    //     if self.db.get_mfr_id(&mfr.name).is_some() {
    //         return Err(EleboxError::AlreadyExists(
    //             "Mfr".to_string(),
    //             mfr.name.clone(),
    //         ));
    //     }

    //     let db_mfr = self.to_db_mfr(mfr)?;
    //     self.db.add_mfr(&db_mfr);
    //     Ok(())
    // }

    // pub fn update(&self, ori_name: &str, new_mfr: &Manufacturer) -> Result<(), EleboxError> {
    //     if self.db.get_mfr_id(ori_name).is_none() {
    //         return Err(EleboxError::NotExists(
    //             "Origin mfr".to_string(),
    //             ori_name.to_string(),
    //         ));
    //     }

    //     if ori_name != &new_mfr.name && self.db.get_mfr_id(&new_mfr.name).is_some() {
    //         return Err(EleboxError::AlreadyExists(
    //             "Mfr".to_string(),
    //             new_mfr.name.clone(),
    //         ));
    //     }

    //     let db_mfr = self.to_db_mfr(new_mfr)?;
    //     self.db.update_mfr(ori_name, &db_mfr);
    //     Ok(())
    // }

    fn to_item(&self, db_item: &DbManufacturer) -> Manufacturer {
        Manufacturer::new(
            &db_item.name,
            Some(db_item.alias.as_str()).filter(|s| !s.is_empty()),
            Some(db_item.url.as_str()).filter(|s| !s.is_empty()),
        )
    }

    // pub fn get(&self, name: &str) -> Result<Manufacturer, EleboxError> {
    //     let id = self
    //         .db
    //         .get_mfr_id(name)
    //         .ok_or(EleboxError::NotExists("Mfr".to_string(), name.to_string()))?;

    //     let db_mfr = self
    //         .db
    //         .get_mfr_from_id(&id)
    //         .ok_or(EleboxError::NotExists("Mfr".to_string(), id))?;

    //     Ok(self.to_mfr(&db_mfr))
    // }

    // pub fn list(&self) -> Vec<Manufacturer> {
    //     self.db
    //         .get_mfrs()
    //         .into_iter()
    //         .filter_map(|db_m| Some(self.to_mfr(&db_m)))
    //         .collect()
    // }

    // pub fn export(&self, filename: &str) -> Result<(), ()> {
    //     let mfrs = self.list();
    //     let res = write_yaml(filename, mfrs);
    //     return res;
    // }

    // pub fn import(&self, filename: &str) -> Result<(), ()> {
    //     let res_parts = read_yaml(filename);
    //     if res_parts.is_err() {
    //         return Err(());
    //     }

    //     let mfrs: Vec<Manufacturer> = res_parts.unwrap();
    //     for mfr in mfrs {
    //         let _ = self.add(&mfr);
    //     }

    //     Ok(())
    // }
}
