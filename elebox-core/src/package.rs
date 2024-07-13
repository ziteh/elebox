use crate::{csv::*, db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, vec};

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
            alias: match alias {
                Some(s) => Some(s.to_string()),
                _ => None,
            },
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
        let id = self.db.get_package_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        self.db.delete_package(&id.unwrap());
        return Ok(());
    }

    pub fn add(&self, item: &Package) -> Result<(), EleboxError> {
        if self.db.get_package_id(&item.name).is_some() {
            return Err(EleboxError::AlreadyExists(item.name.to_string()));
        }

        let db_pkg = DbPackage {
            name: item.name.to_string(),
            pkg_type: match item.pkg_type {
                PackageType::Smt => String::from("smt"),
                PackageType::Tht => String::from("tht"),
                PackageType::Others => String::from("others"),
            },
            alias: match &item.alias {
                Some(s) => s.to_string(),
                _ => String::from(""),
            },
        };

        self.db.add_package(&db_pkg);
        return Ok(());
    }

    pub fn list(&self) -> Vec<Package> {
        let db_pkgs = self.db.get_packages();
        let mut pkgs: Vec<Package> = Vec::new();

        for db_pkg in db_pkgs {
            let p = Package::new(
                &db_pkg.name,
                match db_pkg.pkg_type.as_str() {
                    "smt" => PackageType::Smt,
                    "tht" => PackageType::Tht,
                    "others" => PackageType::Others,
                    _ => panic!(),
                },
                Some(db_pkg.alias.as_str()).filter(|s| !s.is_empty()),
            );
            pkgs.push(p)
        }
        return pkgs;
    }

    pub fn save_csv(&self, filename: &str) -> Result<(), ()> {
        let separator = Some("\t");
        let header = vec!["id", "package_type", "name", "alias"];
        let _ = create_csv(filename, header, separator);

        let packages = self.list();
        for pkg in packages {
            let id = self.db.get_package_id(&pkg.name);
            let pkg_type = match pkg.pkg_type {
                PackageType::Smt => "SMT",
                PackageType::Tht => "THT",
                PackageType::Others => "Others",
            };
            let row = vec![
                id.unwrap(),
                pkg_type.to_string(),
                pkg.name,
                pkg.alias.unwrap_or("".to_string()),
            ];
            let _ = append_csv(filename, &row, separator);
        }

        Ok(())
    }
}
