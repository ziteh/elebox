use crate::{category, comm::*, csv::*, db::*, errors::*, package, transfer::*, yaml::*};
use jammdb::BucketName;
use serde::{Deserialize, Serialize};
use std::{
    env::set_current_dir,
    fmt::{format, Debug},
};

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

pub struct PartManager {
    db: Box<dyn BaseDatabase<DbPart>>,
    pkg_db: Box<dyn BaseDatabase<DbPackage>>,
    mfr_db: Box<dyn BaseDatabase<DbManufacturer>>,
    cat_db: Box<dyn BaseDatabase<DbCategory>>,
}

impl Manager<Part> for PartManager {
    fn init(&self) -> Result<(), EleboxError> {
        let _ = self.pkg_db.init()?;
        let _ = self.mfr_db.init()?;
        let _ = self.cat_db.init()?;
        let _ = self.db.init()?;
        Ok(())
    }

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

        let db_item = self.to_db_part(item)?;
        let _ = self.db.add(&db_item)?;
        Ok(())
    }

    fn update(&self, ori_name: &str, new_item: &Part) -> Result<(), EleboxError> {
        let id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_PART),
                new_item.name.clone(),
            ));
        }

        let db_part = self.to_db_part(new_item)?;
        let _ = self.db.update(ori_name, &db_part)?;
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Part, EleboxError> {
        let id = self.db.get_id(name)?;
        let db_part = self.db.get(&id)?;
        self.from_db_part(db_part)
    }

    fn list(&self) -> Result<Vec<Part>, EleboxError> {
        let db_items = self.db.list()?;
        let mut items: Vec<Part> = vec![];
        for db_item in db_items {
            match self.from_db_part(db_item) {
                Ok(item) => items.push(item),
                Err(err) => return Err(err),
            }
        }
        Ok(items)
    }
}

impl PartManager {
    pub fn new(path: &str) -> Self {
        Self {
            db: Box::new(JammDatabase::new(path, PARTS_BUCKET)),
            pkg_db: Box::new(JammDatabase::new(path, PACKAGES_BUCKET)),
            mfr_db: Box::new(JammDatabase::new(path, MFR_BUCKET)),
            cat_db: Box::new(JammDatabase::new(path, CATEGORIES_BUCKET)),
        }
    }

    // pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
    //     let id = self
    //         .db
    //         .get_id(name)
    //         .ok_or(EleboxError::NotExists("Part".to_string(), name.to_string()))?;

    //     self.db.delete(&id).unwrap();
    //     return Ok("ok".to_string());
    // }

    fn to_db_part(&self, item: &Part) -> Result<DbPart, EleboxError> {
        let category_id = self.cat_db.get_id(&item.category)?;

        let package_id = match &item.package {
            Some(name) => match self.pkg_db.get_id(&name) {
                Ok(id) => id,
                Err(err) => match err {
                    DbError::NotExists(_) => {
                        return Err(EleboxError::NotExists(
                            String::from(ITEM_PKG),
                            name.to_string(),
                        ))
                    }
                    _ => return Err(EleboxError::DatabaseError(err)),
                },
            },
            None => "".to_string(),
        };

        let mfr_id = match &item.mfr {
            Some(name) => match self.mfr_db.get_id(&name) {
                Ok(id) => id,
                Err(err) => match err {
                    DbError::NotExists(_) => {
                        return Err(EleboxError::NotExists(
                            String::from(ITEM_MFR),
                            name.to_string(),
                        ))
                    }
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

    // pub fn update(&self, ori_name: &str, new_part: &Part) -> Result<(), EleboxError> {
    //     todo!();
    //     // if self.db.get_part_id(ori_name).is_none() {
    //     //     return Err(EleboxError::NotExists(
    //     //         "Origin part".to_string(),
    //     //         ori_name.to_string(),
    //     //     ));
    //     // }

    //     // if ori_name != &new_part.name && self.db.get_part_id(&new_part.name).is_some() {
    //     //     return Err(EleboxError::AlreadyExists(
    //     //         "Part".to_string(),
    //     //         new_part.name.clone(),
    //     //     ));
    //     // }

    //     // let db_part = self.to_db_part(new_part)?;
    //     // self.db.update_part(ori_name, &db_part);
    //     // Ok(())
    // }

    pub fn update_part_quantity(&self, name: &str, increment: i16) -> Result<(), EleboxError> {
        todo!();
        // let id = self.db.get_part_id(name);
        // if id.is_none() {
        //     return Err(EleboxError::NotExists("Part".to_string(), name.to_string()));
        // }

        // let mut db_part = self.db.get_part_from_id(id.as_ref().unwrap()).unwrap();
        // let new_q = db_part.quantity as i16 + increment;
        // if new_q < 0 {
        //     return Err(EleboxError::InventoryShortage(name.to_string()));
        // } else {
        //     db_part.quantity = new_q as u16;
        // }

        // self.db.update_part(name, &db_part);
        // return Ok(());
    }

    // pub fn add(&self, part: &Part) -> Result<(), EleboxError> {
    //     // if self.db.get_part_id(&part.name).is_some() {
    //     //     return Err(EleboxError::AlreadyExists(
    //     //         "Part".to_string(),
    //     //         part.name.clone(),
    //     //     ));
    //     // }

    //     // let db_part = self.to_db_part(part)?;
    //     self.db.init();

    //     let dp = DbPart {
    //         name: "test".to_string(),
    //         quantity: 22,
    //         category_id: "asdf".to_string(),
    //         package_id: "".to_string(),
    //         package_detail: "".to_string(),
    //         mfr_id: "".to_string(),
    //         alias: "".to_string(),
    //         description: "".to_string(),
    //         location: "".to_string(),
    //         mfr_no: "".to_string(),
    //         datasheet_link: "".to_string(),
    //         product_link: "".to_string(),
    //         image_link: "".to_string(),
    //         custom_fields: vec![],
    //         suppliers: vec![],
    //         starred: false,
    //     };
    //     self.db.add(&dp);
    //     Ok(())
    // }

    fn from_db_part(&self, db_part: DbPart) -> Result<Part, EleboxError> {
        let category = match self.cat_db.get(&db_part.category_id) {
            Ok(item) => item.name,
            Err(err) => match err {
                DbError::NotExists(_) => {
                    return Err(EleboxError::NotExists(
                        String::from(ITEM_CAT),
                        db_part.category_id,
                    ))
                }
                _ => return Err(EleboxError::DatabaseError(err)),
            },
        };

        let package = match self.pkg_db.get(&db_part.package_id) {
            Ok(item) => Some(item.name),
            Err(err) => match err {
                DbError::NotExists(_) => None,
                _ => return Err(EleboxError::DatabaseError(err)),
            },
        };

        let mfr = match self.mfr_db.get(&db_part.mfr_id) {
            Ok(item) => Some(item.name),
            Err(err) => match err {
                DbError::NotExists(_) => None,
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

        return Ok(part);
    }

    // pub fn get(&self, name: &str) -> Result<Part, EleboxError> {
    //     todo!();
    //     // let id = self
    //     //     .db
    //     //     .get_part_id(name)
    //     //     .ok_or(EleboxError::NotExists("Part".to_string(), name.to_string()))?;

    //     // let db_part = self
    //     //     .db
    //     //     .get_part_from_id(&id)
    //     //     .ok_or(EleboxError::NotExists("Part".to_string(), id))?;

    //     // self.from_db_part(db_part)
    // }

    // pub fn list(&self) -> Vec<Part> {
    //     // self.db
    //     //     .get_parts()
    //     //     .into_iter()
    //     //     .filter_map(|db_p| self.from_db_part(db_p).ok())
    //     //     .collect()
    //     let dbs = self.db.list();
    //     let mut parts: Vec<Part> = vec![];
    //     for d in dbs {
    //         let pp = Part::new(&d.name, "cat", 122);
    //         parts.push(pp);
    //     }
    //     parts
    // }
}

// impl Transferable for PartManager {
//     fn export(&self, filename: &str) -> Result<(), EleboxError> {
//         Ok(())
//         // let parts = self.list();
//         // let res = write_yaml(filename, parts);
//         // return res;
//     }

//     fn import(&self, filename: &str) -> Result<(), EleboxError> {
//         // let res_parts = read_yaml(filename);

//         // if res_parts.is_err() {
//         //     panic!();
//         // }

//         // let parts: Vec<Part> = res_parts.unwrap();
//         // for part in parts {
//         //     if let Err(e) = self.add(&part) {
//         //         panic!("{}", e.to_string())
//         //     }
//         // }

//         Ok(())
//     }
// }
