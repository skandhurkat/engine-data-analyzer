use super::UnitPrinter;

pub enum Angle {
    Degree,
}

impl UnitPrinter for Angle {
    fn format_unit(&self) -> &'static str {
        match self {
            Angle::Degree => "°",
        }
    }
}
