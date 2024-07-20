use crate::{csv::*, db::*, errors::EleboxError, yaml::*};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

const ROOT_CATEGORY: &str = "root";

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

pub struct CategoryManager<'a> {
    db: &'a dyn Database,
}

impl<'a> CategoryManager<'a> {
    pub fn new(db: &'a dyn Database) -> Self {
        Self { db }
    }

    fn to_category(&self, db_category: DbCategory) -> Category {
        let db_parent = self.db.get_category_from_id(&db_category.parent_id);
        Category {
            name: db_category.name,
            parent: db_parent.map(|p_cat| p_cat.name),
            alias: match db_category.alias.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
        }
    }

    pub fn list(&self) -> Vec<Category> {
        self.db
            .get_categories()
            .into_iter()
            .filter_map(|db_c| Some(self.to_category(db_c)))
            .collect()
    }

    pub fn get(&self, name: &str) -> Result<Category, EleboxError> {
        let id = self.db.get_category_id(name).ok_or(EleboxError::NotExists(
            "Category".to_string(),
            name.to_string(),
        ))?;

        let db_cat = self
            .db
            .get_category_from_id(&id)
            .ok_or(EleboxError::NotExists("Category".to_string(), id))?;

        Ok(self.to_category(db_cat))
    }

    pub fn delete(&self, name: &str) -> Result<String, EleboxError> {
        let id = self.db.get_category_id(name).ok_or(EleboxError::NotExists(
            "Category".to_string(),
            name.to_string(),
        ))?;

        Ok(self.db.delete_category(&id))
    }

    pub fn update(&self, ori_name: &str, new_category: &Category) -> Result<(), EleboxError> {
        if self.db.get_category_id(ori_name).is_none() {
            return Err(EleboxError::NotExists(
                "Origin category".to_string(),
                ori_name.to_string(),
            ));
        }

        if ori_name != &new_category.name && self.db.get_category_id(&new_category.name).is_some() {
            return Err(EleboxError::AlreadyExists(
                "Category".to_string(),
                new_category.name.clone(),
            ));
        }

        // Normalize
        let mut cat = Category {
            name: new_category.name.clone(),
            alias: new_category.alias.clone(),
            parent: match &new_category.parent {
                Some(p) => match p.as_str() {
                    "" => None,
                    _ => Some(p.to_string()),
                },
                None => None,
            },
        };

        let db_category = self.to_db_category(&cat)?;
        self.db.update_category(ori_name, &db_category);
        Ok(())
    }

    fn to_db_category(&self, category: &Category) -> Result<DbCategory, EleboxError> {
        // Get the ID of parent category
        let p_id = match &category.parent {
            Some(p_name) => match self.db.get_category_id(&p_name) {
                Some(id) => id,
                None => {
                    return Err(EleboxError::NotExists(
                        "Parent category".to_string(),
                        p_name.to_string(),
                    ))
                }
            },
            None => ROOT_CATEGORY.to_string(),
        };

        let db_category = DbCategory {
            name: category.name.to_string(),
            parent_id: p_id,
            alias: match &category.alias {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        };
        Ok(db_category)
    }

    pub fn add(&self, category: &Category) -> Result<(), EleboxError> {
        // Part category name is unique
        if self.db.get_category_id(&category.name).is_some() {
            return Err(EleboxError::AlreadyExists(
                "Category".to_string(),
                category.name.clone(),
            ));
        }

        // Normalize
        let mut cat = Category {
            name: category.name.clone(),
            alias: category.alias.clone(),
            parent: match &category.parent {
                Some(p) => match p.as_str() {
                    "" => None,
                    _ => Some(p.to_string()),
                },
                None => None,
            },
        };

        let db_category = self.to_db_category(&cat)?;
        self.db.add_category(&db_category);
        Ok(())
    }

    pub fn export(&self, filename: &str) -> Result<(), ()> {
        let cats = self.list();
        let res = write_yaml(filename, cats);
        return res;
    }

    pub fn import(&self, filename: &str) -> Result<(), ()> {
        let res_parts = read_yaml(filename);
        if res_parts.is_err() {
            return Err(());
        }

        let cats: Vec<Category> = res_parts.unwrap();
        for cat in &cats {
            self.add_recursion(cat, &cats);
        }

        Ok(())
    }

    fn add_recursion(&self, category: &Category, cats: &[Category]) -> Result<(), EleboxError> {
        if let Some(parent_name) = &category.parent {
            if let Some(parent_cat) = cats.iter().find(|c| c.name == *parent_name) {
                self.add_recursion(parent_cat, cats);
            }
        }

        self.add(category)
    }

    fn to_node(&self, name: String, map: &HashMap<String, Vec<String>>) -> TreeNode {
        let mut children = vec![];

        if let Some(ch_names) = map.get(&name) {
            for ch_name in ch_names {
                let ch_node = self.to_node(ch_name.to_string(), map);
                children.push(ch_node);
            }
        }

        TreeNode { name, children }
    }

    pub fn get_tree(&self) -> Vec<TreeNode> {
        let cats = self.list();

        let mut cat_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut root: Vec<String> = Vec::new();

        for c in cats {
            if let Some(parent) = c.parent {
                cat_map.entry(parent).or_default().push(c.name);
            } else {
                cat_map.entry(c.name.clone()).or_insert_with(Vec::new);
                root.push(c.name);
            }
        }

        let mut tree_nodes: Vec<TreeNode> = Vec::new();
        for r in root {
            let node = self.to_node(r.to_string(), &cat_map);
            tree_nodes.push(node);
        }

        return tree_nodes;
    }
}
