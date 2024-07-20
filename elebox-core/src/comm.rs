use serde::{ser::SerializeMap, Deserialize, Serialize};

use crate::{Database, EleboxError};

pub const ITEM_PART: String = String::from("part");
pub const ITEM_CAT: String = String::from("category");
pub const ITEM_PKG: String = String::from("package");
pub const ITEM_MFR: String = String::from("manufacturer");

pub type Id = String;

pub trait DbItem {
    fn get_name(&self) -> String;
}

pub trait Manager<T> {
    fn delete(&self, name: &str) -> Result<(), EleboxError>;
    fn add(&self, item: &T) -> Result<(), EleboxError>;
    fn update(&self, ori_name: &str, new_item: &T) -> Result<(), EleboxError>;
    fn get(&self, name: &str) -> Result<T, EleboxError>;
    fn list(&self) -> Result<Vec<T>, EleboxError>;
}

pub trait Transferable {
    fn export(&self, filename: &str) -> Result<(), EleboxError>;
    fn import(&self, filename: &str) -> Result<(), EleboxError>;
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum CustomFieldType {
    Normal,
    Link,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CustomField {
    pub name: String,
    pub field_type: CustomFieldType,
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Supplier {
    pub name: String,
    pub link: String,
    pub price: Option<f32>, // TODO: change to 'rust_decimal'?
    pub note: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct DbPart {
    pub name: String,
    pub quantity: u16,
    pub category_id: Id,
    pub package_id: Id,
    pub package_detail: String,
    pub mfr_id: Id,
    pub alias: String,
    pub description: String,
    pub location: String,
    pub mfr_no: String,
    pub datasheet_link: String,
    pub product_link: String,
    pub image_link: String,
    pub custom_fields: Vec<CustomField>,
    pub suppliers: Vec<Supplier>,
    pub starred: bool,
}

impl DbItem for DbPart {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DbCategory {
    pub name: String,
    pub parent_id: Id,
    pub alias: String,
}

impl DbItem for DbCategory {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbPackage {
    pub pkg_type: String,
    pub name: String,
    pub alias: String,
}

impl DbItem for DbPackage {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DbManufacturer {
    pub name: String,
    pub alias: String,
    pub url: String,
}

impl DbItem for DbManufacturer {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
}
