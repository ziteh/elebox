use crate::{comm::*, jamm_db::*, errors::EleboxError};
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
    db: Box<dyn Database<DbCategory>>,
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

    fn add_recursion(&self, _category: &Category, _cats: &[Category]) -> Result<(), EleboxError> {
        todo!();
        // if let Some(parent_name) = &category.parent {
        //     if let Some(parent_cat) = cats.iter().find(|c| c.name == *parent_name) {
        //         self.add_recursion(parent_cat, cats);
        //     }
        // }

        // self.add(category)
    }

    fn to_node(&self, _name: String, _map: &HashMap<String, Vec<String>>) -> TreeNode {
        todo!();
        // let mut children = vec![];

        // if let Some(ch_names) = map.get(&name) {
        //     for ch_name in ch_names {
        //         let ch_node = self.to_node(ch_name.to_string(), map);
        //         children.push(ch_node);
        //     }
        // }

        // TreeNode { name, children }
    }

    pub fn get_tree(&self) -> Vec<TreeNode> {
        todo!();
        // let cats = self.list();

        // let mut cat_map: HashMap<String, Vec<String>> = HashMap::new();
        // let mut root: Vec<String> = Vec::new();

        // for c in cats {
        //     if let Some(parent) = c.parent {
        //         cat_map.entry(parent).or_default().push(c.name);
        //     } else {
        //         cat_map.entry(c.name.clone()).or_insert_with(Vec::new);
        //         root.push(c.name);
        //     }
        // }

        // let mut tree_nodes: Vec<TreeNode> = Vec::new();
        // for r in root {
        //     let node = self.to_node(r.to_string(), &cat_map);
        //     tree_nodes.push(node);
        // }

        // return tree_nodes;
    }
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
        let cat = Category {
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
        let _id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_CAT),
                new_item.name.clone(),
            ));
        }

        // Normalize
        let cat = Category {
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
