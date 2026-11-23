use crate::units::*;
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
                trace!("{field_name} = {unit_str}");
                if let Ok(unit) = parse_unit_str(unit_str) {
                    trace!("{field_name} ⇒ {:?}", unit);
                } else {
                    panic!("Not implemented: {}", unit_str)
                }
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

fn parse_unit_str(unit_str: &str) -> Result<Measurement, ()> {
    if unit_str == "deg" {
        Ok(Measurement::Angle(AngleUnit::Degree))
    } else if unit_str == "feet" || unit_str == "ft" {
        Ok(Measurement::Length(LengthUnit::Foot))
    } else if unit_str == "NM" {
        Ok(Measurement::Length(LengthUnit::NauticalMile))
    } else if unit_str == "knots" {
        Ok(Measurement::Velocity(VelocityUnit::Knot))
    } else if unit_str == "ft/min" {
        Ok(Measurement::Velocity(VelocityUnit::FootPerMinute))
    } else if unit_str == "deg/s" {
        Ok(Measurement::AngularVelocity(AngularVelocityUnit::DegreePerSecond))
    } else if unit_str == "g" {
        Ok(Measurement::Acceleration(AccelerationUnit::StandardGravity))
    } else if unit_str == "%" {
        Ok(Measurement::Ratio(RatioUnit::Percent))
    } else if unit_str == "steps" || unit_str == "bool" {
        Ok(Measurement::Ratio(RatioUnit::Ratio))
    } else if unit_str == "deg C" {
        Ok(Measurement::Temperature(TemperatureUnit::DegreeCelsius))
    } else if unit_str == "deg F" {
        Ok(Measurement::Temperature(TemperatureUnit::DegreeFahrenheit))
    } else if unit_str == "inHg" {
        Ok(Measurement::Pressure(PressureUnit::InchOfMercury))
    } else {
        unimplemented!("Not implemented");
    }
}
