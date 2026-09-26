use super::UnitPrinter;

pub enum Pressure {
    PSI,
    Pascal,
    KPa,
    Bar,
    InHg,
}

impl Pressure {
    fn to_psi(&self, pressure: f32) -> f32 {
        match self {
            Pressure::PSI => pressure,
            Pressure::Pascal => pressure / 6894.75729,
            Pressure::KPa => pressure / 6.89475729,
            Pressure::Bar => pressure / 0.0689475729,
            Pressure::InHg => pressure * 0.4911541522,
        }
    }

    fn to_pascal(&self, pressure: f32) -> f32 {
        match self {
            Pressure::PSI => pressure * 6894.75729,
            Pressure::Pascal => pressure,
            Pressure::KPa => pressure * 1e3,
            Pressure::Bar => pressure * 1e5,
            Pressure::InHg => pressure * 3386.38867,
        }
    }

    fn to_kpa(&self, pressure: f32) -> f32 {
        match self {
            Pressure::KPa => pressure,
            Pressure::Bar => pressure * 100.,
            _ => self.to_pascal(pressure) / 1e3,
        }
    }

    fn to_bar(&self, pressure: f32) -> f32 {
        match self {
            Pressure::KPa => pressure / 100.,
            Pressure::Bar => pressure,
            _ => self.to_pascal(pressure) / 1e5,
        }
    }

    fn to_in_hg(&self, pressure: f32) -> f32 {
        match self {
            Pressure::PSI => pressure / 0.4911541522,
            Pressure::Pascal => pressure / 3386.38867,
            Pressure::KPa => pressure / 3.38638867,
            Pressure::Bar => pressure / 0.0338638867,
            Pressure::InHg => pressure,
        }
    }
}

impl UnitPrinter for Pressure {
    fn format_unit(&self) -> &'static str {
        match self {
            Pressure::PSI => "psi",
            Pressure::Pascal => "Pa",
            Pressure::KPa => "kPa",
            Pressure::Bar => "bar",
            Pressure::InHg => "inHg",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{F32Margin, assert_approx_eq};
    use parameterized::parameterized;

    #[parameterized(
        psi = {168.3108, 779.3904, 873.1065, 712.8993, 285.3360},
        pascal = {1160462.1136, 5373707.6343, 6019857.3970, 4915267.6385, 1967322.4632},
        kpa = {1160.4621, 5373.7076, 6019.8574, 4915.2676, 1967.3225},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
        in_hg = {342.6842, 1586.8549, 1777.6628, 1451.4777, 580.9500},
    )]
    fn psi_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32, in_hg: f32) {
        let p = Pressure::PSI;
        let p_psi = p.to_psi(psi);
        let p_pascal = p.to_pascal(psi);
        let p_kpa = p.to_kpa(psi);
        let p_bar = p.to_bar(psi);
        let p_in_hg = p.to_in_hg(psi);
        assert_approx_eq!(
            f32,
            p_psi,
            psi,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_pascal,
            pascal,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_kpa,
            kpa,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_bar,
            bar,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_in_hg,
            in_hg,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
    }

    #[parameterized(
        psi = {168.3108, 779.3904, 873.1065, 712.8993, 285.3360},
        pascal = {1160462.1136, 5373707.6343, 6019857.3970, 4915267.6385, 1967322.4632},
        kpa = {1160.4621, 5373.7076, 6019.8574, 4915.2676, 1967.3225},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
        in_hg = {342.6842, 1586.8549, 1777.6628, 1451.4777, 580.9500},
    )]
    fn pascal_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32, in_hg: f32) {
        let p = Pressure::Pascal;
        let p_psi = p.to_psi(pascal);
        let p_pascal = p.to_pascal(pascal);
        let p_kpa = p.to_kpa(pascal);
        let p_bar = p.to_bar(pascal);
        let p_in_hg = p.to_in_hg(pascal);
        assert_approx_eq!(
            f32,
            p_psi,
            psi,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_pascal,
            pascal,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_kpa,
            kpa,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_bar,
            bar,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_in_hg,
            in_hg,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
    }

    #[parameterized(
        psi = {168.3108, 779.3904, 873.1065, 712.8993, 285.3360},
        pascal = {1160462.1000, 5373707.6000, 6019857.3000, 4915267.6000, 1967322.4000},
        kpa = {1160.4621, 5373.7076, 6019.8574, 4915.2676, 1967.3225},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
        in_hg = {342.6842, 1586.8549, 1777.6628, 1451.4777, 580.9500},
    )]
    fn kpa_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32, in_hg: f32) {
        let p = Pressure::KPa;
        let p_psi = p.to_psi(kpa);
        let p_pascal = p.to_pascal(kpa);
        let p_kpa = p.to_kpa(kpa);
        let p_bar = p.to_bar(kpa);
        let p_in_hg = p.to_in_hg(kpa);
        assert_approx_eq!(
            f32,
            p_psi,
            psi,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_pascal,
            pascal,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_kpa,
            kpa,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_bar,
            bar,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );

        assert_approx_eq!(
            f32,
            p_in_hg,
            in_hg,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
    }

    #[parameterized(
        psi = {168.3105, 779.3908, 873.1069, 712.8997, 285.3357},
        pascal = {1160460.0000, 5373710.0000, 6019860.0000, 4915270.0000, 1967320.0000},
        kpa = {1160.4600, 5373.7100, 6019.8600, 4915.2700, 1967.3200},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
        in_hg = {342.6836, 1586.8557, 1777.6636, 1451.4784, 580.9493},
    )]
    fn bar_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32, in_hg: f32) {
        let p = Pressure::Bar;
        let p_psi = p.to_psi(bar);
        let p_pascal = p.to_pascal(bar);
        let p_kpa = p.to_kpa(bar);
        let p_bar = p.to_bar(bar);
        let p_in_hg = p.to_in_hg(bar);
        assert_approx_eq!(
            f32,
            p_psi,
            psi,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_pascal,
            pascal,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_kpa,
            kpa,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_bar,
            bar,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
        assert_approx_eq!(
            f32,
            p_in_hg,
            in_hg,
            F32Margin {
                epsilon: 0.0001,
                ulps: 4
            }
        );
    }
}
