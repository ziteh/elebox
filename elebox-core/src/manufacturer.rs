use crate::{csv::*, db::*, errors::EleboxError};
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

pub struct ManufacturerManager<'a> {
    db: &'a dyn Database,
}

impl<'a> ManufacturerManager<'a> {
    pub fn new(db: &'a dyn Database) -> Self {
        Self { db }
    }

    pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
        let id = self
            .db
            .get_mfr_id(name)
            .ok_or(EleboxError::NotExists("Mfr".to_string(), name.to_string()))?;

        Ok(self.db.delete_mfr(&id))
    }

    fn to_db_mfr(&self, mfr: &Manufacturer) -> Result<DbManufacturer, EleboxError> {
        if self.db.get_mfr_id(&mfr.name).is_some() {
            return Err(EleboxError::AlreadyExists(
                "Mfr".to_string(),
                mfr.name.clone(),
            ));
        }

        let db_mfr = DbManufacturer {
            name: mfr.name.to_string(),
            alias: match &mfr.alias {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            url: match &mfr.url {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        };
        Ok(db_mfr)
    }

    pub fn add(&self, mfr: &Manufacturer) -> Result<(), EleboxError> {
        let db_mfr = self.to_db_mfr(mfr)?;
        self.db.add_mfr(&db_mfr);
        Ok(())
    }

    pub fn update(&self, ori_name: &str, new_mfr: &Manufacturer) -> Result<(), EleboxError> {
        if self.db.get_mfr_id(ori_name).is_none() {
            return Err(EleboxError::NotExists(
                "Origin mfr".to_string(),
                ori_name.to_string(),
            ));
        }

        let db_mfr = self.to_db_mfr(new_mfr)?;
        self.db.update_mfr(ori_name, &db_mfr);
        Ok(())
    }

    fn to_mfr(&self, db_mfr: &DbManufacturer) -> Manufacturer {
        Manufacturer::new(
            &db_mfr.name,
            Some(db_mfr.alias.as_str()).filter(|s| !s.is_empty()),
            Some(db_mfr.url.as_str()).filter(|s| !s.is_empty()),
        )
    }

    pub fn get(&self, name: &str) -> Result<Manufacturer, EleboxError> {
        let id = self
            .db
            .get_mfr_id(name)
            .ok_or(EleboxError::NotExists("Mfr".to_string(), name.to_string()))?;

        let db_mfr = self
            .db
            .get_mfr_from_id(&id)
            .ok_or(EleboxError::NotExists("Mfr".to_string(), id))?;

        Ok(self.to_mfr(&db_mfr))
    }

    pub fn list(&self) -> Vec<Manufacturer> {
        self.db
            .get_mfrs()
            .into_iter()
            .filter_map(|db_m| Some(self.to_mfr(&db_m)))
            .collect()
    }

    pub fn export_csv(&self, filename: &str) -> Result<(), ()> {
        let mfrs = self.list();
        let res = write_csv(filename, mfrs, None);
        return res;
    }

    pub fn import_csv(&self, filename: &str) -> Result<(), ()> {
        let res_parts = read_csv(filename, None);
        if res_parts.is_err() {
            return Err(());
        }

        let mfrs: Vec<Manufacturer> = res_parts.unwrap();
        for mfr in mfrs {
            let _ = self.add(&mfr);
        }

        Ok(())
    }
}
