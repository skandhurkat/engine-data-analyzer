mod length;
mod pressure;
mod speed;
mod temperature;
mod angle;

pub use length::*;
pub use pressure::*;
pub use speed::*;
pub use temperature::*;
pub use angle::*;

pub enum Unit {
    Temperature(temperature::Temperature),
    Pressure(pressure::Pressure),
    Length(length::Length),
    Speed(speed::Speed),
    Angle(angle::Angle),
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
            Unit::Speed(speed) => speed.format_unit(),
            Unit::Angle(angle) => angle.format_unit(),
        }
    }
}
