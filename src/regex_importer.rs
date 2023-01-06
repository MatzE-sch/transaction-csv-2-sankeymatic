use csv::ReaderBuilder;
use csv::StringRecord;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct RegLab {
    pub regex: String,
    pub labels: String,
}

pub fn read_regex_labels(file_name: String) -> Result<Vec<RegLab>, Box<dyn Error>> {
    let mut regex_table = Vec::new();

    let mut rdr = ReaderBuilder::new().from_path(file_name)?;
    // rdr.set_headers(header);

    for result in rdr.records() {
        let record = result?;
        let header = StringRecord::from(vec!["regex", "labels"]);
        let row: RegLab = record.deserialize(Some(&header))?;
        regex_table.push(row);
    }
    Ok(regex_table)
}
