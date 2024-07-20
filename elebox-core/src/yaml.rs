use std::fs::{self, File};

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

use serde_yaml;

pub fn write_yaml<T>(filename: &str, items: Vec<T>) -> Result<(), ()>
where
    T: Serialize,
{
    let contents = serde_yaml::to_string(&items).unwrap();
    fs::write(filename, contents);
    Ok(())
}

pub fn read_yaml<T>(filename: &str) -> Result<Vec<T>, ()>
where
    T: for<'a> Deserialize<'a>,
{
    let content = fs::read_to_string(filename).unwrap();
    let items: Vec<T> = serde_yaml::from_str(&content).unwrap();
    Ok(items)
}
