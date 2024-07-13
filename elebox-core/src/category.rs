use crate::{csv::*, db::*, errors::EleboxError};
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
            parent: match parent {
                Some(p) => Some(p.to_string()),
                None => None,
            },
            alias: match alias {
                Some(a) => Some(a.to_string()),
                None => None,
            },
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
                alias: match db_cat.alias.as_str() {
                    "" => None,
                    other => Some(other.to_string()),
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

        let category = Category {
            name: db_cat.name.to_string(),
            parent: match self.db.get_category_from_id(&db_cat.parent_id) {
                Some(cat) => Some(cat.name),
                None => None,
            },
            alias: match db_cat.alias.as_str() {
                "" => None,
                other => Some(other.to_string()),
            },
        };
        return Ok(category);
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
            alias: old_db_pt.alias,
        };

        self.db.add_category(&db_category);
        return Ok(());
    }

    pub fn add(&self, category: &Category) -> Result<(), EleboxError> {
        // Part category name is unique
        if self.db.get_category_id(&category.name).is_some() {
            return Err(EleboxError::AlreadyExists(category.name.to_string()));
        }

        // Get the ID of parent category
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
            alias: match &category.alias {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        };

        self.db.add_category(&db_category);
        return Ok(());
    }

    pub fn export_csv(&self, filename: &str) -> Result<(), ()> {
        let cats = self.list();
        let res = write_csv(filename, cats, None);
        return res;
    }

    pub fn import_csv(&self, filename: &str) -> Result<(), ()> {
        let res_parts = read_csv(filename, None);
        if res_parts.is_err() {
            return Err(());
        }

        let cats: Vec<Category> = res_parts.unwrap();
        for cat in cats {
            let _ = self.add(&cat);
        }

        Ok(())
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

        // TODO: test data
        // let cats = vec![
        //     Category {
        //         name: "a1".to_string(),
        //         parent: Some("A".to_string()),
        //     },
        //     Category {
        //         name: "aa1".to_string(),
        //         parent: Some("a1".to_string()),
        //     },
        //     Category {
        //         name: "aa2".to_string(),
        //         parent: Some("a1".to_string()),
        //     },
        //     Category {
        //         name: "aaa1".to_string(),
        //         parent: Some("aa1".to_string()),
        //     },
        //     Category {
        //         name: "aab2".to_string(),
        //         parent: Some("a2".to_string()),
        //     },
        //     Category {
        //         name: "a2".to_string(),
        //         parent: Some("A".to_string()),
        //     },
        //     Category {
        //         name: "b1".to_string(),
        //         parent: Some("B".to_string()),
        //     },
        //     Category {
        //         name: "A".to_string(),
        //         parent: None,
        //     },
        //     Category {
        //         name: "B".to_string(),
        //         parent: None,
        //     },
        // ];
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
