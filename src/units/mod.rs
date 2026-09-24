pub trait Unit {
    fn format_unit(&self) -> &'static str;
}

mod pressure;
mod temperature;

pub use pressure::*;
pub use temperature::*;
