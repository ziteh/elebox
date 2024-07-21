use crate::{comm::*, csv::*, db::*, errors::EleboxError, yaml::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

const ROOT_CATEGORY: &str = "__root__";

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub parent: Option<String>,
    pub alias: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

impl Category {
    pub fn new(name: &str, parent: Option<&str>, alias: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            parent: parent.map(|p| p.to_string()),
            alias: alias.map(|a| a.to_string()),
        }
    }
}

pub struct CategoryManager {
    db: Box<dyn BaseDatabase<DbCategory>>,
}

impl Manager<Category> for CategoryManager {
    fn init(&self) -> Result<(), EleboxError> {
        let _ = self.db.init()?;
        Ok(())
    }

    fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_id(name)?;
        let _ = self.db.delete(&id)?;
        Ok(())
    }

    fn add(&self, item: &Category) -> Result<(), EleboxError> {
        if self.db.get_id(&item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_CAT),
                item.name.clone(),
            ));
        }

        // Normalize
        let mut cat = Category {
            name: item.name.clone(),
            alias: item.alias.clone(),
            parent: match &item.parent {
                Some(p) => match p.as_str() {
                    "" => None,
                    _ => Some(p.to_string()),
                },
                None => None,
            },
        };
        let db_item = self.to_db_category(&cat)?;
        let _ = self.db.add(&db_item)?;
        Ok(())
    }

    fn update(&self, ori_name: &str, new_item: &Category) -> Result<(), EleboxError> {
        let id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_CAT),
                new_item.name.clone(),
            ));
        }

        // Normalize
        let mut cat = Category {
            name: new_item.name.clone(),
            alias: new_item.alias.clone(),
            parent: match &new_item.parent {
                Some(p) => match p.as_str() {
                    "" => None,
                    _ => Some(p.to_string()),
                },
                None => None,
            },
        };

        let db_item = self.to_db_category(&cat)?;
        let _ = self.db.update(ori_name, &db_item)?;
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Category, EleboxError> {
        let id = self.db.get_id(name)?;
        let db_item = self.db.get(&id)?;
        Ok(self.to_item(db_item))
    }

    fn list(&self) -> Result<Vec<Category>, EleboxError> {
        let db_items = self.db.list()?;
        let mut items: Vec<Category> = vec![];
        for db_item in db_items {
            items.push(self.to_item(db_item));
        }
        Ok(items)
    }
}

impl CategoryManager {
    pub fn new(path: &str) -> Self {
        Self {
            db: Box::new(JammDatabase::new(path, CATEGORIES_BUCKET)),
        }
    }

    fn to_item(&self, db_category: DbCategory) -> Category {
        let db_parent = match self.db.get(&db_category.parent_id) {
            Ok(db_item) => Some(db_item.name),
            Err(_) => None,
        };

        Category {
            name: db_category.name,
            parent: db_parent,
            alias: match db_category.alias.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
        }
    }

    // pub fn list(&self) -> Vec<Category> {
    //     self.db
    //         .get_categories()
    //         .into_iter()
    //         .filter_map(|db_c| Some(self.to_category(db_c)))
    //         .collect()
    // }

    // pub fn get(&self, name: &str) -> Result<Category, EleboxError> {
    //     let id = self.db.get_category_id(name).ok_or(EleboxError::NotExists(
    //         "Category".to_string(),
    //         name.to_string(),
    //     ))?;

    //     let db_cat = self
    //         .db
    //         .get_category_from_id(&id)
    //         .ok_or(EleboxError::NotExists("Category".to_string(), id))?;

    //     Ok(self.to_category(db_cat))
    // }

    // pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
    //     let id = self.db.get_category_id(name).ok_or(EleboxError::NotExists(
    //         "Category".to_string(),
    //         name.to_string(),
    //     ))?;

    //     Ok(self.db.delete_category(&id))
    // }

    // pub fn update(&self, ori_name: &str, new_category: &Category) -> Result<(), EleboxError> {
    //     if self.db.get_category_id(ori_name).is_none() {
    //         return Err(EleboxError::NotExists(
    //             "Origin category".to_string(),
    //             ori_name.to_string(),
    //         ));
    //     }

    //     if ori_name != &new_category.name && self.db.get_category_id(&new_category.name).is_some() {
    //         return Err(EleboxError::AlreadyExists(
    //             "Category".to_string(),
    //             new_category.name.clone(),
    //         ));
    //     }

    //     // Normalize
    //     let mut cat = Category {
    //         name: new_category.name.clone(),
    //         alias: new_category.alias.clone(),
    //         parent: match &new_category.parent {
    //             Some(p) => match p.as_str() {
    //                 "" => None,
    //                 _ => Some(p.to_string()),
    //             },
    //             None => None,
    //         },
    //     };

    //     let db_category = self.to_db_category(&cat)?;
    //     self.db.update_category(ori_name, &db_category);
    //     Ok(())
    // }

    fn to_db_category(&self, item: &Category) -> Result<DbCategory, EleboxError> {
        // Get the ID of parent category

        let parent_id = match &item.parent {
            Some(parent_name) => self.db.get_id(parent_name)?,
            None => ROOT_CATEGORY.to_string(),
        };

        let db_category = DbCategory {
            name: item.name.to_string(),
            parent_id,
            alias: match &item.alias {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        };
        Ok(db_category)
    }

    // pub fn add(&self, category: &Category) -> Result<(), EleboxError> {
    //     // Part category name is unique
    //     if self.db.get_category_id(&category.name).is_some() {
    //         return Err(EleboxError::AlreadyExists(
    //             "Category".to_string(),
    //             category.name.clone(),
    //         ));
    //     }

    //     // Normalize
    //     let mut cat = Category {
    //         name: category.name.clone(),
    //         alias: category.alias.clone(),
    //         parent: match &category.parent {
    //             Some(p) => match p.as_str() {
    //                 "" => None,
    //                 _ => Some(p.to_string()),
    //             },
    //             None => None,
    //         },
    //     };

    //     let db_category = self.to_db_category(&cat)?;
    //     self.db.add_category(&db_category);
    //     Ok(())
    // }

    // pub fn export(&self, filename: &str) -> Result<(), ()> {
    //     let cats = self.list();
    //     let res = write_yaml(filename, cats);
    //     return res;
    // }

    // pub fn import(&self, filename: &str) -> Result<(), ()> {
    //     let res_parts = read_yaml(filename);
    //     if res_parts.is_err() {
    //         return Err(());
    //     }

    //     let cats: Vec<Category> = res_parts.unwrap();
    //     for cat in &cats {
    //         self.add_recursion(cat, &cats);
    //     }

    //     Ok(())
    // }

    // fn add_recursion(&self, category: &Category, cats: &[Category]) -> Result<(), EleboxError> {
    //     if let Some(parent_name) = &category.parent {
    //         if let Some(parent_cat) = cats.iter().find(|c| c.name == *parent_name) {
    //             self.add_recursion(parent_cat, cats);
    //         }
    //     }

    //     self.add(category)
    // }

    // fn to_node(&self, name: String, map: &HashMap<String, Vec<String>>) -> TreeNode {
    //     let mut children = vec![];

    //     if let Some(ch_names) = map.get(&name) {
    //         for ch_name in ch_names {
    //             let ch_node = self.to_node(ch_name.to_string(), map);
    //             children.push(ch_node);
    //         }
    //     }

    //     TreeNode { name, children }
    // }

    // pub fn get_tree(&self) -> Vec<TreeNode> {
    //     let cats = self.list();

    //     let mut cat_map: HashMap<String, Vec<String>> = HashMap::new();
    //     let mut root: Vec<String> = Vec::new();

    //     for c in cats {
    //         if let Some(parent) = c.parent {
    //             cat_map.entry(parent).or_default().push(c.name);
    //         } else {
    //             cat_map.entry(c.name.clone()).or_insert_with(Vec::new);
    //             root.push(c.name);
    //         }
    //     }

    //     let mut tree_nodes: Vec<TreeNode> = Vec::new();
    //     for r in root {
    //         let node = self.to_node(r.to_string(), &cat_map);
    //         tree_nodes.push(node);
    //     }

    //     return tree_nodes;
    // }
}
