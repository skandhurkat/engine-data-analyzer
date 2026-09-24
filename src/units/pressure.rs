pub enum Pressure {
    PSI(f32),
    Pascal(f32),
    KPa(f32),
    Bar(f32),
}

impl Pressure {
    fn to_psi(&self) -> f32 {
        match self {
            Pressure::PSI(p) => *p,
            Pressure::Pascal(p) => p / 6894.75729,
            Pressure::KPa(p) => p / 6.89475729,
            Pressure::Bar(p) => p / 0.0689475729,
        }
    }

    fn to_pascal(&self) -> f32 {
        match self {
            Pressure::PSI(p) => p * 6894.75729,
            Pressure::Pascal(p) => *p,
            Pressure::KPa(p) => p * 1e3,
            Pressure::Bar(p) => p * 1e5,
        }
    }

    fn to_kpa(&self) -> f32 {
        match self {
            Pressure::KPa(p) => *p,
            Pressure::Bar(p) => p * 100.,
            _ => self.to_pascal() / 1e3,
        }
    }

    fn to_bar(&self) -> f32 {
        match self {
            Pressure::KPa(p) => p / 100.,
            Pressure::Bar(p) => *p,
            _ => self.to_pascal() / 1e5,
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
    )]
    fn psi_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32) {
        let p = Pressure::PSI(psi);
        let p_psi = p.to_psi();
        let p_pascal = p.to_pascal();
        let p_kpa = p.to_kpa();
        let p_bar = p.to_bar();
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
    }

    #[parameterized(
        psi = {168.3108, 779.3904, 873.1065, 712.8993, 285.3360},
        pascal = {1160462.1136, 5373707.6343, 6019857.3970, 4915267.6385, 1967322.4632},
        kpa = {1160.4621, 5373.7076, 6019.8574, 4915.2676, 1967.3225},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
    )]
    fn pascal_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32) {
        let p = Pressure::Pascal(pascal);
        let p_psi = p.to_psi();
        let p_pascal = p.to_pascal();
        let p_kpa = p.to_kpa();
        let p_bar = p.to_bar();
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
    }

    #[parameterized(
        psi = {168.3108, 779.3904, 873.1065, 712.8993, 285.3360},
        pascal = {1160462.1000, 5373707.6000, 6019857.3000, 4915267.6000, 1967322.4000},
        kpa = {1160.4621, 5373.7076, 6019.8574, 4915.2676, 1967.3225},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
    )]
    fn kpa_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32) {
        let p = Pressure::KPa(kpa);
        let p_psi = p.to_psi();
        let p_pascal = p.to_pascal();
        let p_kpa = p.to_kpa();
        let p_bar = p.to_bar();
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
    }

    #[parameterized(
        psi = {168.3105, 779.3908, 873.1069, 712.8997, 285.3357},
        pascal = {1160460.0000, 5373710.0000, 6019860.0000, 4915270.0000, 1967320.0000},
        kpa = {1160.4600, 5373.7100, 6019.8600, 4915.2700, 1967.3200},
        bar = {11.6046, 53.7371, 60.1986, 49.1527, 19.6732},
    )]
    fn bar_conversion_test(psi: f32, pascal: f32, kpa: f32, bar: f32) {
        let p = Pressure::Bar(bar);
        let p_psi = p.to_psi();
        let p_pascal = p.to_pascal();
        let p_kpa = p.to_kpa();
        let p_bar = p.to_bar();
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
    }
}
