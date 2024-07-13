use std::fs::{File, OpenOptions};
use std::io::{self, Write};

pub fn write_csv(filename: &str, data: &[Vec<String>], separator: Option<&str>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let sep = separator.unwrap_or(",");

    for row in data {
        let line = row.join(sep);
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn create_csv(filename: &str, header: Vec<&str>, separator: Option<&str>) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let sep = separator.unwrap_or(",");
    let line = header.join(sep);
    writeln!(file, "{}", line)?;
    Ok(())
}

pub fn append_csv(filename: &str, data: &[String], separator: Option<&str>) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;

    let sep = separator.unwrap_or(",");
    let line = data.join(sep);
    writeln!(file, "{}", line)?;

    Ok(())
}
