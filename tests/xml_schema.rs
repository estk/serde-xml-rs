extern crate serde_xml_rs;
use serde_xml_rs::from_reader;
use serde_xml_rs::schema::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_print_xml_schema() {
    read_fixture("XMLSchema.xsd").unwrap();
}
#[test]
fn test_print_gpx() {
    read_fixture("gpx.xsd").unwrap();
}

fn read_fixture(filename: &str) -> Result<Schema, Box<dyn Error>> {
    let path = format!("./tests/fixtures/{}", filename);
    let reader = BufReader::new(File::open(path)?);
    let schema: Schema = from_reader(reader)?;

    Ok(schema)
}
