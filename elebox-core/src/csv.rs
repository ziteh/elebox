use std::fs::File;

use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub fn write_csv<T>(filename: &str, items: Vec<T>, separator: Option<u8>) -> Result<(), ()>
where
    T: Serialize,
{
    let sep = separator.unwrap_or(b'\t');
    let res_file = File::create(filename);
    if res_file.is_err() {
        return Err(());
    }

    let mut writer = WriterBuilder::new()
        .delimiter(sep)
        .from_writer(res_file.unwrap());

    for item in items {
        let res = writer.serialize(item);
        if res.is_err() {
            return Err(());
        }
    }
    let res = writer.flush();
    if res.is_err() {
        return Err(());
    }

    Ok(())
}

#[allow(dead_code)]
pub fn read_csv<T>(filename: &str, separator: Option<u8>) -> Result<Vec<T>, ()>
where
    T: for<'a> Deserialize<'a>,
{
    let sep = separator.unwrap_or(b'\t');
    let res_file = File::open(filename);
    if res_file.is_err() {
        return Err(());
    }

    let mut reader = ReaderBuilder::new()
        .delimiter(sep)
        .from_reader(res_file.unwrap());

    let mut items = Vec::new();
    for res in reader.deserialize() {
        if res.is_err() {
            return Err(());
        }
        let item: T = res.unwrap();
        items.push(item)
    }

    return Ok(items);
}
