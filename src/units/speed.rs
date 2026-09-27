use super::UnitPrinter;

pub enum Speed {
    Kph,
    Mph,
    Kts,
    Fpm,
}

impl UnitPrinter for Speed {
    fn format_unit(&self) -> &'static str {
        match self {
            Speed::Kph => "km/h",
            Speed::Mph => "sm/h",
            Speed::Kts => "kts",
            Speed::Fpm => "ft/s",
        }
    }
}
