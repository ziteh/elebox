use crate::{comm::*, errors::EleboxError, jamm_db::*, yaml::*};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Manufacturer {
    pub name: String,
    pub alias: Option<String>,
    pub url: Option<String>,
}

impl Manufacturer {
    pub fn new(name: &str, alias: Option<&str>, url: Option<&str>) -> Self {
        Self {
            name: name.to_string(),
            alias: alias.map(|s| s.to_string()),
            url: url.map(|s| s.to_string()),
        }
    }
}

pub struct ManufacturerHandler<'a> {
    pub(crate) db: &'a dyn Database<DbManufacturer>,
}

impl ManufacturerHandler<'_> {
    fn to_db_item(&self, item: &Manufacturer) -> Result<DbManufacturer, EleboxError> {
        let db_mfr = DbManufacturer {
            name: item.name.to_string(),
            alias: match &item.alias {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
            url: match &item.url {
                Some(s) => s.to_string(),
                None => "".to_string(),
            },
        };
        Ok(db_mfr)
    }

    fn to_item(&self, db_item: &DbManufacturer) -> Manufacturer {
        Manufacturer::new(
            &db_item.name,
            Some(db_item.alias.as_str()).filter(|s| !s.is_empty()),
            Some(db_item.url.as_str()).filter(|s| !s.is_empty()),
        )
    }
}

impl<'a> Handler<Manufacturer> for ManufacturerHandler<'_> {
    fn delete(&self, name: &str) -> Result<(), EleboxError> {
        let id = self.db.get_id(name)?;
        let _ = self.db.delete(&id)?;
        Ok(())
    }

    fn add(&self, item: &Manufacturer) -> Result<(), EleboxError> {
        if self.db.get_id(&item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_MFR),
                item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(item)?;
        let _ = self.db.add(&db_item)?;
        Ok(())
    }

    fn update(&self, ori_name: &str, new_item: &Manufacturer) -> Result<(), EleboxError> {
        let ori_id = self.db.get_id(ori_name)?;

        if ori_name != &new_item.name && self.db.get_id(&new_item.name).is_ok() {
            return Err(EleboxError::AlreadyExists(
                String::from(ITEM_MFR),
                new_item.name.clone(),
            ));
        }

        let db_item = self.to_db_item(new_item)?;
        let _ = self.db.update(ori_id.as_str(), &db_item)?;
        Ok(())
    }

    fn get(&self, name: &str) -> Result<Manufacturer, EleboxError> {
        let id = self.db.get_id(name)?;
        let db_item = self.db.get(&id)?;
        Ok(self.to_item(&db_item))
    }

    fn list(&self) -> Result<Vec<Manufacturer>, EleboxError> {
        let db_items = self.db.list()?;
        let mut items: Vec<Manufacturer> = vec![];
        for db_item in db_items {
            items.push(self.to_item(&db_item));
        }
        Ok(items)
    }
}

impl Transferable for ManufacturerHandler<'_> {
    fn export(&self, filename: &PathBuf) -> Result<(), EleboxError> {
        let items = self.list()?;
        let _ = write_yaml(filename, items).unwrap();
        Ok(())
    }

    fn import(&self, filename: &PathBuf) -> Result<(), EleboxError> {
        let res_items = read_yaml(filename);

        if res_items.is_err() {
            todo!()
        }

        let items: Vec<Manufacturer> = res_items.unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DbError;
    use mockall::{mock, predicate::*};

    mock! {
        pub MyDatabase {}

        impl Database<DbManufacturer> for MyDatabase {
            fn init(&self) -> Result<(), DbError>;
            fn get_id(&self, name: &str) -> Result<String, DbError>;
            fn get(&self, id: &str) -> Result<DbManufacturer, DbError>;
            fn list(&self) -> Result<Vec<DbManufacturer>, DbError>;
            fn add(&self, item: &DbManufacturer) -> Result<(), DbError>;
            fn update(&self, ori_id: &str, new_item: &DbManufacturer) -> Result<(), DbError>;
            fn delete(&self, id: &str) -> Result<(), DbError>;
            fn check(&self) -> Result<(), DbError>;
        }
    }

    #[test]
    fn test_add_no_existing() {
        const SYMBOL: &str = r"_=+-!@#$%^&*()|{}/\'`~";

        let test_cases = vec![
            ("Test1", Some("TestAlias"), Some("https://test.com")),
            ("Test2", None, Some("https://test.com")),
            ("Test3", Some("TestAlias"), None),
            ("Test4", None, None),
            ("Test Space", Some("Test Alias"), Some("https://test.com")),
            (SYMBOL, Some(SYMBOL), Some(SYMBOL)),
            ("中文名稱", Some("中文別名"), Some("中文網址")),
        ];

        for (name, alias, url) in test_cases {
            test_add_no_existing_value(
                name.to_string(),
                alias.map(|s| s.to_string()),
                url.map(|s| s.to_string()),
            );
        }
    }

    fn test_add_no_existing_value(name: String, alias: Option<String>, url: Option<String>) {
        // Arrange
        let mfr = Manufacturer::new(
            &name.clone(),
            alias.clone().as_deref(),
            url.clone().as_deref(),
        );

        let mut mock_db = MockMyDatabase::new();

        // Mock get_id() always return NotExists indicating the item does not existing
        mock_db
            .expect_get_id()
            .returning(|_| Err(DbError::NotExists("ID".to_string())));

        // Mock add() to check all field and return Ok(())
        mock_db
            .expect_add()
            .withf(move |db_mfr: &DbManufacturer| {
                db_mfr.name == name
                    && db_mfr.alias == alias.as_deref().unwrap_or("")
                    && db_mfr.url == url.as_deref().unwrap_or("")
            })
            .returning(|_| Ok(()));

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.add(&mfr);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_existing() {
        // Arrange
        const NAME: &str = "TestName";
        const ID: &str = "TestID";

        let mut mock_db = MockMyDatabase::new();

        // Mock get_id() to check name, and return ID indicating the item existing
        mock_db
            .expect_get_id()
            .withf(|name| name == NAME)
            .returning(|_| Ok(ID.to_string()));

        // Mock delete() to check ID, and return Ok(())
        mock_db
            .expect_delete()
            .withf(|id| id == ID)
            .returning(|_| Ok(()));

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.delete(NAME);

        // Assert
        assert!(result.is_ok());
    }
}
