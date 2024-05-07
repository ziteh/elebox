use crate::{db::*, errors::EleboxError};
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
            alias: match alias {
                Some(s) => Some(s.to_string()),
                _ => None,
            },
            url: match url {
                Some(s) => Some(s.to_string()),
                _ => None,
            },
        }
    }
}

pub struct ManufacturerManager<'a> {
    db: &'a dyn Datebase,
}

impl<'a> ManufacturerManager<'a> {
    pub fn new(db: &'a dyn Datebase) -> Self {
        Self { db }
    }

    pub fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_mfr_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        self.db.delete_mfr(&id.unwrap());
        return Ok(());
    }

    pub fn add(&self, item: &Manufacturer) -> Result<(), EleboxError> {
        if self.db.get_mfr_id(&item.name).is_some() {
            return Err(EleboxError::AlreadyExists(item.name.to_string()));
        }

        let db_mfr = DbManufacturer {
            name: item.name.to_string(),
            alias: match &item.alias {
                Some(s) => s.to_string(),
                _ => String::from(""),
            },
            url: match &item.url {
                Some(s) => s.to_string(),
                _ => String::from(""),
            },
        };

        self.db.add_mfr(&db_mfr);
        return Ok(());
    }

    pub fn list(&self) -> Vec<Manufacturer> {
        let db_mfrs = self.db.get_mfrs();
        let mut mfrs: Vec<Manufacturer> = Vec::new();

        for db_mfr in db_mfrs {
            let m = Manufacturer::new(
                &db_mfr.name,
                Some(db_mfr.alias.as_str()).filter(|s| !s.is_empty()),
                Some(db_mfr.url.as_str()).filter(|s| !s.is_empty()),
            );
            mfrs.push(m);
        }
        return mfrs;
    }
}
