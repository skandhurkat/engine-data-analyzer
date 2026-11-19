use arrow::array::builder::ArrayBuilder;
use arrow::datatypes::{Schema, SchemaBuilder};
use regex::Regex;
use spdlog::prelude::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::{Arc, LazyLock};

pub struct ParsedData {}

pub fn try_parse_dynon<P>(file_path: P) -> std::io::Result<ParsedData>
where
    P: AsRef<Path>,
{
    let mut file_buf = open_file(file_path)?;
    let _ = parse_header(&mut file_buf)?;
    Ok(ParsedData {})
}

fn open_file<P>(file_path: P) -> std::io::Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

struct Parser {
    schema: Arc<Schema>,
    builders: Vec<Box<dyn ArrayBuilder>>,
    parsers: Vec<Box<dyn Fn(&str)>>,
    column_factory: Vec<Box<dyn Fn()>>,
}

fn parse_header(file_buf: &mut BufReader<File>) -> std::io::Result<()> {
    let mut lines = file_buf.lines();

    let Some(Ok(header)) = lines.next() else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not read header from file",
        ));
    };

    let fields = header.split(',').map(|s| s.trim());

    let mut schema_builder = SchemaBuilder::new();
    for (i, f) in fields.enumerate() {
        if i == 0 && f != "Session Time" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Not a Dynon file, does not start with Session Time as the first field",
            ));
        }
        if f == "" {
            continue;
        }
        let Some((field_name, unit)) = split_name_and_unit_for_field(f) else {
            error!("Could not parse {f}");
            panic!("Could not parse {f}");
        };
        match unit {
            Some(unit_str) => {
                trace!("{field_name} ⇒ {unit_str}");
            }
            None => {
                trace!("{field_name} ⇒ no unit");
            }
        }
    }
    Ok(())
}

fn split_name_and_unit_for_field(field: &str) -> Option<(&str, Option<&str>)> {
    static SPLIT_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(.*?)\s*(?:\((.*?)\))?$").unwrap());
    let captures = SPLIT_REGEX.captures(field).unwrap();
    assert!(captures.len() <= 3, "Assertion {} <= 3", captures.len());
    let field_name: &str = captures.get(1)?.as_str();
    let unit: Option<&str> = captures.get(2).map_or(None, |m| Some(m.as_str()));
    Some((field_name, unit))
}
