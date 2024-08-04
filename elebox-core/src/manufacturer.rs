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
            test_add_not_existing_value(
                name.to_string(),
                alias.map(|s| s.to_string()),
                url.map(|s| s.to_string()),
            );
        }
    }

    fn test_add_not_existing_value(name: String, alias: Option<String>, url: Option<String>) {
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
            .withf(move |db_mfr| {
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

    #[test]
    fn test_get_existing() {
        // Arrange
        const ID: &str = "TestID";
        const NAME: &str = "TestName";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";

        let mut mock_db = MockMyDatabase::new();

        // Mock get_id() to check name, and return ID indicating the item existing
        mock_db
            .expect_get_id()
            .withf(|name| name == NAME)
            .returning(|_| Ok(ID.to_string()));

        // Mock get() to check ID, and return item
        mock_db.expect_get().withf(|id| id == ID).returning(|_| {
            Ok(DbManufacturer {
                name: NAME.to_string(),
                alias: ALIAS.to_string(),
                url: URL.to_string(),
            })
        });

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.get(NAME);

        // Assert
        let mfr = result.expect("Expected OK");
        assert_eq!(mfr.name, NAME);
        assert_eq!(mfr.alias.as_deref(), Some(ALIAS));
        assert_eq!(mfr.url.as_deref(), Some(URL));
    }

    #[test]
    fn test_list_existing() {
        // Arrange
        const NAME: &str = "TestName";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";
        const TEST_NUMBER: u16 = 10;

        let mut mock_db = MockMyDatabase::new();

        // Mock list() return items
        mock_db.expect_list().returning(|| {
            let data: Vec<DbManufacturer> = (0..TEST_NUMBER)
                .map(|i| DbManufacturer {
                    name: format!("{NAME}{i}"),
                    alias: format!("{ALIAS}{i}"),
                    url: format!("{URL}{i}"),
                })
                .collect();
            Ok(data)
        });

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.list();

        // Assert
        let mfrs = result.expect("Expected OK");
        for (i, m) in mfrs.iter().enumerate() {
            assert_eq!(m.name, format!("{NAME}{i}"));
            assert_eq!(m.alias, Some(format!("{ALIAS}{i}")));
            assert_eq!(m.url, Some(format!("{URL}{i}")));
        }
    }

    /// Update item, name does not change
    #[test]
    fn test_update_not_existing() {
        // Arrange
        const NAME: &str = "TestName";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";
        const ID: &str = "TestID";

        let mut mock_db = MockMyDatabase::new();

        // Mock get_id() to check name, and return ID indicating the item existing
        mock_db
            .expect_get_id()
            .withf(|name| name == NAME)
            .returning(|_| Ok(ID.to_string()));

        // Mock update() check all fields and return Ok
        mock_db
            .expect_update()
            .withf(|ori_id, new_item| {
                ori_id == ID
                    && new_item.name == NAME
                    && new_item.alias == ALIAS
                    && new_item.url == URL
            })
            .returning(|_, _| Ok(()));

        // Act
        let new_item = Manufacturer {
            name: NAME.to_string(),
            alias: Some(ALIAS.to_string()),
            url: Some(URL.to_string()),
        };

        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.update(NAME, &new_item);

        // Assert
        assert!(result.is_ok());
    }

    /// Update item, name changed and not duplicate
    #[test]
    fn test_update_new_name_not_existing() {
        // Arrange
        const ORI_NAME: &str = "TestNameOri";
        const NEW_NAME: &str = "TestNameNew";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";
        const ID: &str = "TestID";

        let mut mock_db = MockMyDatabase::new();

        // Mock get_id() to check name, and return ID indicating the item existing
        mock_db
            .expect_get_id()
            .withf(|name| name == ORI_NAME)
            .returning(|_| Ok(ID.to_string()));

        // New item new does not existing
        mock_db
            .expect_get_id()
            .withf(|name| name == NEW_NAME)
            .returning(|_| Err(DbError::NotExists(NEW_NAME.to_string())));

        // Mock update() check all fields and return Ok
        mock_db
            .expect_update()
            .withf(|ori_id, new_item| {
                ori_id == ID
                    && new_item.name == NEW_NAME
                    && new_item.alias == ALIAS
                    && new_item.url == URL
            })
            .returning(|_, _| Ok(()));

        // Act
        let new_item = Manufacturer {
            name: NEW_NAME.to_string(),
            alias: Some(ALIAS.to_string()),
            url: Some(URL.to_string()),
        };

        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.update(ORI_NAME, &new_item);

        // Assert
        assert!(result.is_ok());
    }

    /// Update item, name changed and duplicate
    #[test]
    fn test_update_new_name_existing() {
        // Arrange
        const ORI_NAME: &str = "TestNameOri";
        const NEW_NAME: &str = "TestNameNew";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";
        const ORI_ID: &str = "TestIDOri";
        const NEW_ID: &str = "TestIDNew";

        let mut mock_db = MockMyDatabase::new();

        // Check original item existing
        mock_db
            .expect_get_id()
            .withf(|name| name == ORI_NAME)
            .returning(|_| Ok(ORI_ID.to_string()));

        // New item name existing
        mock_db
            .expect_get_id()
            .withf(|name| name == NEW_NAME)
            .returning(|_| Ok(NEW_ID.to_string()));

        // Act
        let new_item = Manufacturer {
            name: NEW_NAME.to_string(),
            alias: Some(ALIAS.to_string()),
            url: Some(URL.to_string()),
        };

        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.update(ORI_NAME, &new_item);

        // Assert
        if let Err(EleboxError::AlreadyExists(_, _)) = result {
            // Ok
        } else {
            panic!("Expected AlreadyExists error")
        }
    }

    #[test]
    fn test_to_db_item_some() {
        // Arrange
        const NAME: &str = "TestName";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";

        let item = Manufacturer {
            name: NAME.to_string(),
            alias: Some(ALIAS.to_string()),
            url: Some(URL.to_string()),
        };

        let mock_db = MockMyDatabase::new();

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.to_db_item(&item);

        // Assert
        let db_item = result.expect("Expected OK");
        assert_eq!(db_item.name, NAME.to_string());
        assert_eq!(db_item.alias, ALIAS.to_string());
        assert_eq!(db_item.url, URL.to_string());
    }

    #[test]
    fn test_to_db_item_none() {
        // Arrange
        const NAME: &str = "TestName";
        const ALIAS: &str = "";
        const URL: &str = "";

        let item = Manufacturer {
            name: NAME.to_string(),
            alias: None,
            url: None,
        };

        let mock_db = MockMyDatabase::new();

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let result = handler.to_db_item(&item);

        // Assert
        let db_item = result.expect("Expected OK");
        assert_eq!(db_item.name, NAME.to_string());
        assert_eq!(db_item.alias, ALIAS.to_string());
        assert_eq!(db_item.url, URL.to_string());
    }

    #[test]
    fn test_to_item_some() {
        // Arrange
        const NAME: &str = "TestName";
        const ALIAS: &str = "TestAlias";
        const URL: &str = "https://test.com";

        let item = DbManufacturer {
            name: NAME.to_string(),
            alias: ALIAS.to_string(),
            url: URL.to_string(),
        };

        let mock_db = MockMyDatabase::new();

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let item = handler.to_item(&item);

        // Assert
        assert_eq!(item.name, NAME.to_string());
        assert_eq!(item.alias, Some(ALIAS.to_string()));
        assert_eq!(item.url, Some(URL.to_string()));
    }

    #[test]
    fn test_to_item_none() {
        // Arrange
        const NAME: &str = "TestName";
        const ALIAS: &str = "";
        const URL: &str = "";

        let item = DbManufacturer {
            name: NAME.to_string(),
            alias: ALIAS.to_string(),
            url: URL.to_string(),
        };

        let mock_db = MockMyDatabase::new();

        // Act
        let handler = ManufacturerHandler { db: &mock_db };
        let item = handler.to_item(&item);

        // Assert
        assert_eq!(item.name, NAME.to_string());
        assert_eq!(item.alias, None);
        assert_eq!(item.url, None);
    }
}
