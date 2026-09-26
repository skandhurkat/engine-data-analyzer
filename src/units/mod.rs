mod length;
mod pressure;
mod temperature;

pub use length::*;
pub use pressure::*;
pub use temperature::*;

pub enum Unit {
    Temperature(temperature::Temperature),
    Pressure(pressure::Pressure),
    Length(length::Length),
}

pub trait UnitPrinter {
    fn format_unit(&self) -> &'static str;
}

impl UnitPrinter for Unit {
    fn format_unit(&self) -> &'static str {
        match self {
            Unit::Temperature(temp) => temp.format_unit(),
            Unit::Pressure(press) => press.format_unit(),
            Unit::Length(len) => len.format_unit(),
        }
    }
}
