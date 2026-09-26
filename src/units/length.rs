use super::UnitPrinter;

pub enum Length {
    KiloMetre,
    StatuteMile,
    NauticalMile,
    Feet,
    Metre,
}

impl UnitPrinter for Length {
    fn format_unit(&self) -> &'static str {
        match self {
            Length::KiloMetre => "km",
            Length::StatuteMile => "sm",
            Length::NauticalMile => "nm",
            Length::Feet => "ft",
            Length::Metre => "m",
        }
    }
}
