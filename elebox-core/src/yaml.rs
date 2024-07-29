use std::{
    fs::{self},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use serde_yaml;

pub fn write_yaml<T>(filename: &PathBuf, items: Vec<T>) -> Result<(), ()>
where
    T: Serialize,
{
    let contents = serde_yaml::to_string(&items).unwrap();
    fs::write(filename, contents).unwrap();
    Ok(())
}

pub fn read_yaml<T>(filename: &PathBuf) -> Result<Vec<T>, ()>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(filename).unwrap();
    let items: Vec<T> = serde_yaml::from_str(&content).unwrap();
    Ok(items)
}
