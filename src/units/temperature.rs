pub trait TemperatureLike {
    fn to_celsius(&self) -> f32;
    fn to_farenheit(&self) -> f32;
}

pub struct Celsius {
    temp: f32,
}
pub struct Farenheit {
    temp: f32,
}

pub enum Temperature {
    Celsius(Celsius),
    Farenheit(Farenheit),
}

impl TemperatureLike for Celsius {
    fn to_celsius(&self) -> f32 {
        self.temp
    }

    fn to_farenheit(&self) -> f32 {
        self.temp * 9.0 / 5.0 + 32.0
    }
}

impl TemperatureLike for Farenheit {
    fn to_celsius(&self) -> f32 {
        (self.temp - 32.0) * 5.0 / 9.0
    }

    fn to_farenheit(&self) -> f32 {
        self.temp
    }
}

impl TemperatureLike for Temperature {
    fn to_celsius(&self) -> f32 {
        match self {
            Temperature::Celsius(c) => c.to_celsius(),
            Temperature::Farenheit(f) => f.to_celsius(),
        }
    }

    fn to_farenheit(&self) -> f32 {
        match self {
            Temperature::Celsius(c) => c.to_farenheit(),
            Temperature::Farenheit(f) => f.to_farenheit(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(c = {0.0, 100.0, -40.0})]
    fn c_to_c_test(c: f32) {
        let c_type = Celsius { temp: c };
        assert_eq!(c_type.to_celsius(), c);
    }

    #[parameterized(c = {0.0, 100.0, -40.0}, f = {32.0, 212.0, -40.0})]
    fn c_to_f_test(c: f32, f: f32) {
        let c_type = Celsius { temp: c };
        assert_eq!(c_type.to_farenheit(), f);
    }

    #[parameterized(f = {32.0, 212.0, -40.0}, c = {0.0, 100.0, -40.0})]
    fn f_to_c_test(f: f32, c: f32) {
        let f_type = Farenheit { temp: f };
        assert_eq!(f_type.to_celsius(), c);
    }

    #[parameterized(f = {32.0, 212.0, -40.0})]
    fn f_to_f_test(f: f32) {
        let f_type = Farenheit { temp: f };
        assert_eq!(f_type.to_farenheit(), f);
    }

    #[parameterized(c = {0.0, 100.0, -40.0})]
    fn enum_c_to_c_test(c: f32) {
        let t: Temperature = Temperature::Celsius(Celsius { temp: c });
        assert_eq!(t.to_celsius(), c);
    }

    #[parameterized(c = {0.0, 100.0, -40.0}, f = {32.0, 212.0, -40.0})]
    fn enum_c_to_f_test(c: f32, f: f32) {
        let t: Temperature = Temperature::Celsius(Celsius { temp: c });
        assert_eq!(t.to_farenheit(), f);
    }

    #[parameterized(f = {32.0, 212.0, -40.0}, c = {0.0, 100.0, -40.0})]
    fn enum_f_to_c_test(f: f32, c: f32) {
        let t: Temperature = Temperature::Farenheit(Farenheit { temp: f });
        assert_eq!(t.to_celsius(), c);
    }

    #[parameterized(f = {32.0, 212.0, -40.0})]
    fn enum_f_to_f_test(f: f32) {
        let t: Temperature = Temperature::Farenheit(Farenheit { temp: f });
        assert_eq!(t.to_farenheit(), f);
    }
}
