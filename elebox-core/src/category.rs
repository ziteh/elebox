use crate::{db::*, errors::EleboxError};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

const ROOT_CATEGORY: &str = "root";

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub parent: Option<String>,
}

pub struct CategoryManager {
    db: Datebase,
}

impl CategoryManager {
    pub fn new(db: Datebase) -> Self {
        Self { db }
    }

    pub fn list(&self) -> Vec<Category> {
        let db_categories = self.db.get_categories();
        let mut categories: Vec<Category> = Vec::new();

        for db_cat in db_categories {
            let db_parent = self.db.get_category_from_id(&db_cat.parent_id);

            categories.push(Category {
                name: db_cat.name,
                parent: match db_parent {
                    Some(p) => Some(p.name),
                    None => None,
                },
            });
        }
        return categories;
    }

    pub fn get(&self, name: &str) -> Result<Category, EleboxError> {
        let id = match self.db.get_category_id(name) {
            Some(id) => id,
            None => return Err(EleboxError::NotExists(name.to_string())),
        };

        let db_cat = match self.db.get_category_from_id(&id) {
            Some(pt) => pt,
            None => return Err(EleboxError::NotExists(id.to_string())),
        };

        let cateogry = Category {
            name: db_cat.name.to_string(),
            parent: match self.db.get_category_from_id(&db_cat.parent_id) {
                Some(cat) => Some(cat.name),
                None => None,
            },
        };
        return Ok(cateogry);
    }

    pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
        let id = self.db.get_category_id(name);
        if id.is_none() {
            return Err(EleboxError::NotExists(name.to_string()));
        }

        let res = self.db.delete_category(&id.unwrap());
        return Ok(res);
    }

    pub fn update(
        &self,
        old_name: &str,
        new_name: Option<&str>,
        new_parent: Option<&str>,
    ) -> Result<(), EleboxError> {
        let id = self.db.get_category_id(old_name);
        if id.is_none() {
            return Err(EleboxError::NotExists(old_name.to_string()));
        }

        let old_db_pt = self.db.get_category_from_id(id.as_ref().unwrap()).unwrap();

        let p_id = match new_parent {
            Some(name) => match self.db.get_category_id(name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(name.to_string())),
            },
            None => old_db_pt.parent_id,
        };

        let db_category = DbCategory {
            name: match new_name {
                Some(name) => name.to_string(),
                None => old_db_pt.name,
            },
            parent_id: p_id,
        };

        self.db.add_category(&db_category);
        return Ok(());
    }

    pub fn add(&self, category: &Category) -> Result<(), EleboxError> {
        // Part type name is unique
        if self.db.get_category_id(&category.name).is_some() {
            return Err(EleboxError::AlreadyExists(category.name.to_string()));
        }

        // Get the ID of parent type
        let p_id = match &category.parent {
            Some(p_name) => match self.db.get_category_id(&p_name) {
                Some(id) => id,
                None => return Err(EleboxError::NotExists(p_name.to_string())),
            },
            None => ROOT_CATEGORY.to_string(),
        };

        let db_category = DbCategory {
            name: category.name.to_string(),
            parent_id: p_id,
        };

        self.db.add_category(&db_category);
        return Ok(());
    }
}
