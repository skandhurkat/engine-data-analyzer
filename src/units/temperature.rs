pub enum Temperature {
    Celsius(f32),
    Farenheit(f32),
}

impl Temperature {
    fn to_celsius(&self) -> f32 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Farenheit(f) => (f - 32.) * 5. / 9.,
        }
    }

    fn to_farenheit(&self) -> f32 {
        match self {
            Temperature::Celsius(c) => c * 9. / 5. + 32.,
            Temperature::Farenheit(f) => *f,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parameterized::parameterized;

    #[parameterized(c = {0.0, 100.0, -40.0})]
    fn c_to_c_test(c: f32) {
        let t = Temperature::Celsius(c);
        assert_eq!(t.to_celsius(), c);
    }

    #[parameterized(c = {0.0, 100.0, -40.0}, f = {32.0, 212.0, -40.0})]
    fn c_to_f_test(c: f32, f: f32) {
        let t = Temperature::Celsius(c);
        assert_eq!(t.to_farenheit(), f);
    }

    #[parameterized(f = {32.0, 212.0, -40.0}, c = {0.0, 100.0, -40.0})]
    fn f_to_c_test(f: f32, c: f32) {
        let t = Temperature::Farenheit(f);
        assert_eq!(t.to_celsius(), c);
    }

    #[parameterized(f = {32.0, 212.0, -40.0})]
    fn f_to_f_test(f: f32) {
        let t = Temperature::Farenheit(f);
        assert_eq!(t.to_farenheit(), f);
    }
}
