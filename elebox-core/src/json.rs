use crate::HumanReadable;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self},
    path::PathBuf,
};

pub struct JsonFile {}

impl HumanReadable for JsonFile {
    fn write<T>(filename: &PathBuf, items: Vec<T>) -> Result<(), ()>
    where
        T: Serialize,
    {
        // TODO unwrap
        let contents = serde_json::to_string(&items).unwrap();
        fs::write(filename, contents).unwrap();
        Ok(())
    }

    fn read<T>(filename: &PathBuf) -> Result<Vec<T>, ()>
    where
        T: for<'de> Deserialize<'de>,
    {
        // TODO unwrap
        let content = fs::read_to_string(filename).unwrap();
        let items: Vec<T> = serde_json::from_str(&content).unwrap();
        Ok(items)
    }

    fn check_extension(filename: &PathBuf) -> bool {
        filename.extension().map_or(false, |ext| ext == "json")
    }
}
