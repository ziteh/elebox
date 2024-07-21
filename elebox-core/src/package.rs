use crate::{
    comm::*,
    jamm_db::*,
    errors::{EleboxError},
};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize)]
pub enum PackageType {
    Smt,
    Tht,
    Others,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub pkg_type: PackageType,
    pub name: String,
    pub alias: Option<String>,
}

impl Package {
    pub fn new(name: &str, pkg_type: PackageType, alias: Option<&str>) -> Self {
        Self {
            pkg_type,
            name: name.to_string(),
            alias: alias.map(|s| s.to_string()),
        }
    }
}

pub struct PackageManager {
    db: Box<dyn Database<DbPackage>>,
}

impl PackageManager {
    pub fn new(path: &str) -> Self {
        Self {
            db: Box::new(JammDatabase::new(path, PACKAGES_BUCKET)),
        }
    }

    fn to_db_item(&self, item: &Package) -> Result<DbPackage, EleboxError> {
        let db_pkg = DbPackage {
            name: item.name.to_string(),
            pkg_type: match item.pkg_type {
                PackageType::Smt => String::from("smt"),
                PackageType::Tht => String::from("tht"),
                PackageType::Others => String::from("others"),
            },
            alias: match &item.alias {
                Some(s) => String::from(s),
                None => String::from(""),
            },
        };
        Ok(db_pkg)
    }

    fn to_item(&self, db_item: DbPackage) -> Package {
        Package {
            name: db_item.name,
            pkg_type: match db_item.pkg_type.as_str() {
                "smt" => PackageType::Smt,
                "tht" => PackageType::Tht,
                _ => PackageType::Others,
            },
            alias: match db_item.alias.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
        }
    }
}

impl Manager<Package> for PackageManager {
    fn init(&self) -> Result<(), EleboxError> {
        let _ = self.db.init()?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_id(name)?;
        let _ = self.db.delete(&id)?;
        Ok(())
    }

    fn add(&self, item: &Package) -> Result<(), EleboxError> {
        if self.db.get_id(&item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_PKG),
                item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(item)?;
        let _ = self.db.add(&db_item)?;
        Ok(())
    }

    fn update(&self, ori_name: &str, new_item: &Package) -> Result<(), EleboxError> {
        let _id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_PKG),
                new_item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(new_item)?;
        let _ = self.db.update(ori_name, &db_item)?;
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Package, EleboxError> {
        let id = self.db.get_id(name)?;
        let db_item = self.db.get(&id)?;
        Ok(self.to_item(db_item))
    }

    fn list(&self) -> Result<Vec<Package>, EleboxError> {
        let db_items = self.db.list()?;
        let mut items: Vec<Package> = vec![];
        for db_item in db_items {
            items.push(self.to_item(db_item));
        }
        Ok(items)
    }
}
