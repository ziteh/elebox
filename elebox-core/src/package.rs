use crate::{comm::*, errors::EleboxError, jamm_db::*, json::*, yaml::*};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub enum PackageType {
    Smt,
    Tht,
    Others,
}

impl fmt::Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PackageType::Smt => "SMT",
            PackageType::Tht => "THT",
            PackageType::Others => "Others",
        };
        write!(f, "{s}")
    }
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

pub struct PackageHandler<'a> {
    pub(crate) db: &'a dyn Database<DbPackage>,
}

impl PackageHandler<'_> {
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

impl<'a> Handler<Package> for PackageHandler<'_> {
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
        let ori_id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_PKG),
                new_item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(new_item)?;
        let _ = self.db.update(ori_id.as_str(), &db_item)?;
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

impl Transferable for PackageHandler<'_> {
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
        let res_items: Result<Vec<Package>, ()> = if YamlFile::check_extension(filename) {
            YamlFile::read(filename)
        } else if JsonFile::check_extension(filename) {
            JsonFile::read(filename)
        } else {
            todo!()
        };

        if res_items.is_err() {
            todo!()
        }

        let items: Vec<Package> = res_items.unwrap();
        for item in items {
            if let Err(e) = self.add(&item) {
                match e {
                    EleboxError::AlreadyExists(_, _) => continue,
                    others => panic!("{}", others.to_string()),
                }
            }
        }

        Ok(())
    }
}
