use super::UnitPrinter;

pub enum Temperature {
    Celsius,
    Farenheit,
}

impl Temperature {
    fn to_celsius(&self, temp: f32) -> f32 {
        match self {
            Temperature::Celsius => temp,
            Temperature::Farenheit => (temp - 32.) * 5. / 9.,
        }
    }

    fn to_farenheit(&self, temp: f32) -> f32 {
        match self {
            Temperature::Celsius => temp * 9. / 5. + 32.,
            Temperature::Farenheit => temp,
        }
    }
}

impl UnitPrinter for Temperature {
    fn format_unit(&self) -> &'static str {
        match self {
            Temperature::Celsius => "℃",
            Temperature::Farenheit => "℉",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{F32Margin, assert_approx_eq};
    use parameterized::parameterized;

    #[parameterized(c = {0.0, 100.0, -40.0}, f = {32.0, 212.0, -40.0})]
    fn c_conversion_test(c: f32, f: f32) {
        let t = Temperature::Celsius;
        let t_c = t.to_celsius(c);
        let t_f = t.to_farenheit(c);

        assert_approx_eq!(
            f32,
            t_c,
            c,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            t_f,
            f,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
    }

    #[parameterized(c = {0.0, 100.0, -40.0}, f = {32.0, 212.0, -40.0})]
    fn f_conversion_test(c: f32, f: f32) {
        let t = Temperature::Farenheit;
        let t_c = t.to_celsius(f);
        let t_f = t.to_farenheit(f);

        assert_approx_eq!(
            f32,
            t_c,
            c,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            t_f,
            f,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
    }
}
