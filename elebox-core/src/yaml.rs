use crate::HumanReadable;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{
    fs::{self},
    path::PathBuf,
};

pub struct YamlFile {}

impl HumanReadable for YamlFile {
    fn write<T>(filename: &PathBuf, items: Vec<T>) -> Result<(), ()>
    where
        T: Serialize,
    {
        // TODO unwrap
        let contents = serde_yaml::to_string(&items).unwrap();
        fs::write(filename, contents).unwrap();
        Ok(())
    }

    fn read<T>(filename: &PathBuf) -> Result<Vec<T>, ()>
    where
        T: for<'de> Deserialize<'de>,
    {
        // TODO unwrap
        let content = fs::read_to_string(filename).unwrap();
        let items: Vec<T> = serde_yaml::from_str(&content).unwrap();
        Ok(items)
    }

    fn check_extension(filename: &PathBuf) -> bool {
        filename.extension().map_or(false, |ext| ext == "yaml")
    }
}
