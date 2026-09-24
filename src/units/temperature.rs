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
    use float_cmp::{F32Margin, assert_approx_eq};
    use parameterized::parameterized;

    #[parameterized(c = {0.0, 100.0, -40.0}, f = {32.0, 212.0, -40.0})]
    fn c_conversion_test(c: f32, f: f32) {
        let t = Temperature::Celsius(c);
        let t_c = t.to_celsius();
        let t_f = t.to_farenheit();

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
        let t = Temperature::Farenheit(f);
        let t_c = t.to_celsius();
        let t_f = t.to_farenheit();

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
