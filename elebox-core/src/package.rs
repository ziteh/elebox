use crate::{csv::*, db::*, errors::EleboxError, yaml::*};
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

pub struct PackageManager<'a> {
    db: &'a dyn Database,
}

impl<'a> PackageManager<'a> {
    pub fn new(db: &'a dyn Database) -> Self {
        Self { db }
    }

    pub fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_package_id(name).ok_or(EleboxError::NotExists(
            "Package".to_string(),
            name.to_string(),
        ))?;

        self.db.delete_package(&id);
        Ok(())
    }

    fn to_db_package(&self, package: &Package) -> Result<DbPackage, EleboxError> {
        let db_pkg = DbPackage {
            name: package.name.to_string(),
            pkg_type: match package.pkg_type {
                PackageType::Smt => String::from("smt"),
                PackageType::Tht => String::from("tht"),
                PackageType::Others => String::from("others"),
            },
            alias: match &package.alias {
                Some(s) => s.to_string(),
                _ => String::from(""),
            },
        };
        Ok(db_pkg)
    }

    pub fn add(&self, package: &Package) -> Result<(), EleboxError> {
        if self.db.get_package_id(&package.name).is_some() {
            return Err(EleboxError::AlreadyExists(
                "Package".to_string(),
                package.name.clone(),
            ));
        }

        let db_pkg = self.to_db_package(package)?;
        self.db.add_package(&db_pkg);
        Ok(())
    }

    pub fn update(&self, ori_name: &str, new_package: &Package) -> Result<(), EleboxError> {
        if self.db.get_package_id(ori_name).is_none() {
            return Err(EleboxError::NotExists(
                "Origin package".to_string(),
                ori_name.to_string(),
            ));
        }

        if ori_name != &new_package.name && self.db.get_package_id(&new_package.name).is_some() {
            return Err(EleboxError::AlreadyExists(
                "Package".to_string(),
                new_package.name.clone(),
            ));
        }

        let db_pkg = self.to_db_package(new_package)?;
        self.db.update_package(ori_name, &db_pkg);
        Ok(())
    }

    fn to_package(&self, db_package: DbPackage) -> Package {
        Package {
            name: db_package.name,
            pkg_type: match db_package.pkg_type.as_str() {
                "smt" => PackageType::Smt,
                "tht" => PackageType::Tht,
                _ => PackageType::Others,
            },
            alias: match db_package.alias.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
        }
    }

    pub fn get(&self, name: &str) -> Result<Package, EleboxError> {
        let id = self.db.get_package_id(name).ok_or(EleboxError::NotExists(
            "Package".to_string(),
            name.to_string(),
        ))?;

        let db_pkg = self
            .db
            .get_package_from_id(&id)
            .ok_or(EleboxError::NotExists("Package".to_string(), id))?;

        Ok(self.to_package(db_pkg))
    }

    pub fn list(&self) -> Vec<Package> {
        self.db
            .get_packages()
            .into_iter()
            .filter_map(|db_p| Some(self.to_package(db_p)))
            .collect()
    }

    pub fn export(&self, filename: &str) -> Result<(), ()> {
        let pkgs = self.list();
        let res = write_yaml(filename, pkgs);
        return res;
    }

    pub fn import(&self, filename: &str) -> Result<(), ()> {
        let res_parts = read_yaml(filename);
        if res_parts.is_err() {
            return Err(());
        }

        let pkgs: Vec<Package> = res_parts.unwrap();
        for pkg in pkgs {
            let _ = self.add(&pkg);
        }

        Ok(())
    }
}
