use regex::Regex;
use spdlog::prelude::{debug, error, info, trace, warn};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;
use std::sync::LazyLock;

pub struct ParsedData {}

pub fn try_parse_dynon<P>(file_path: P) -> Result<ParsedData>
where
    P: AsRef<Path>,
{
    let mut file_buf = open_file(file_path)?;
    parse_header(&mut file_buf);
    Ok(ParsedData {})
}

fn open_file<P>(file_path: P) -> Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn parse_header(file_buf: &mut BufReader<File>) {
    let mut lines = file_buf.lines();

    let Some(Ok(header)) = lines.next() else {
        return;
    };
    let fields = header.split(',').map(|s| s.trim());
    for f in fields {
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
