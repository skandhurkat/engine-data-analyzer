use uom::si::f32::*;

unit! {
system: uom::si;
quantity: uom::si::volume_rate;
@gallon_per_hour: 3.785_412_E-3 / 3.60_E3, 0.; "gal/h", "gallon per hour", "gallons per hour";
@liter_per_hour: 1_E-3 / 3.60_E3, 0.; "L/h", "liter per hour", "liters per hour";
}

macro_rules! define_units {
    (
        $(#[$enum_meta:meta])*
        $enum:ident : $quantity:ty {
            $(
                $variant:ident => $unit:path
            ),* $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        pub enum $enum {
            $(
                $variant,
            )*
        }

        impl $enum {
            fn to_quantity(&self, val: f32) -> $quantity {
                match self {
                    $(
                        Self::$variant => <$quantity>::new::<$unit>(val),
                    )*
                }
            }

            fn from_quantity(&self, val: $quantity) -> f32 {
                match self {
                    $(
                        Self::$variant => val.get::<$unit>(),
                    )*
                }
            }

            pub fn convert(&self, to_unit: &Self, val: f32) -> f32 {
                if *self == *to_unit {
                    return val;
                }
                to_unit.from_quantity(self.to_quantity(val))
            }
        }
    };
}

define_units!(
#[derive(PartialEq)]
AccelerationUnit: Acceleration {
    StandardGravity => uom::si::acceleration::standard_gravity,
}
);

define_units!(
#[derive(PartialEq)]
AngleUnit: Angle {
    Degree => uom::si::angle::degree,
}
);

define_units!(
#[derive(PartialEq)]
AngularVelocityUnit: AngularVelocity {
    DegreePerSecond => uom::si::angular_velocity::degree_per_second,
    RevolutionPerMinute => uom::si::angular_velocity::revolution_per_minute,
}
);

define_units!(
#[derive(PartialEq)]
ElectricCurrentUnit: ElectricCurrent {
    Ampere => uom::si::electric_current::ampere,
}
);

define_units!(
#[derive(PartialEq)]
ElectricPotentialUnit: ElectricPotential {
    Volt => uom::si::electric_potential::volt,
}
);

define_units!(
#[derive(PartialEq)]
LengthUnit: Length {
    Kilometer => uom::si::length::kilometer,
    StatuteMile => uom::si::length::mile,
    NauticalMile => uom::si::length::nautical_mile,
}
);

define_units!(
#[derive(PartialEq)]
PressureUnit: Pressure {
    HectoPascal => uom::si::pressure::pascal,
    InchOfMercury => uom::si::pressure::inch_of_mercury,
    PoundForcePerSquareInch => uom::si::pressure::pound_force_per_square_inch,
}
);

define_units!(
#[derive(PartialEq)]
TemperatureUnit: ThermodynamicTemperature {
    DegreeCelsius => uom::si::thermodynamic_temperature::degree_celsius,
    DegreeFahrenheit => uom::si::thermodynamic_temperature::degree_fahrenheit,
}
);

define_units!(
#[derive(PartialEq)]
VelocityUnit: Velocity {
    FootPerMinute => uom::si::velocity::foot_per_minute,
    KilometerPerHour => uom::si::velocity::kilometer_per_hour,
    Knot => uom::si::velocity::knot,
    StatuteMilePerHour => uom::si::velocity::mile_per_hour,
}
);

define_units!(
#[derive(PartialEq)]
VolumeUnit: Volume {
    Gallon => uom::si::volume::gallon,
    Liter => uom::si::volume::liter,
}
);

define_units!(
#[derive(PartialEq)]
VolumeRateUnit: VolumeRate {
    GallonPerHour => gallon_per_hour,
    LiterPerHour => liter_per_hour,
}
);

macro_rules! measurement {
    (
        $(
            $variant:ident($unit_type:ty)
        ),* $(,)?
    ) => {
        pub enum Measurement {
            $(
                $variant($unit_type),
            )*
        }

        impl Measurement {
            pub fn convert(
                &self,
                to: &Self,
                value: f32,
            ) -> Option<f32> {
                match (self, to) {
                    $(
                        (
                            Self::$variant(from),
                            Self::$variant(to),
                        ) => Some(from.convert(to, value)),
                    )*

                    _ => None,
                }
            }
        }
    };
}

measurement! {
    Acceleration(AccelerationUnit),
    Angle(AngleUnit),
    AngularVelocity(AngularVelocityUnit),
    ElectricCurrent(ElectricCurrentUnit),
    ElectricPotential(ElectricPotentialUnit),
    Length(LengthUnit),
    Pressure(PressureUnit),
    Temperature(TemperatureUnit),
    Velocity(VelocityUnit),
    Volume(VolumeUnit),
    VolumeRate(VolumeRateUnit),
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{F32Margin, approx_eq};

    #[test]
    fn test_accelation_conversion() {
        let m = Measurement::Acceleration(AccelerationUnit::StandardGravity);
        let value: f32 = 0.6;
        let converted = m.convert(
            &Measurement::Acceleration(AccelerationUnit::StandardGravity),
            value,
        );
        assert_ne!(converted, None);
        assert_eq!(converted.unwrap(), value);

        let converted_fail = m.convert(&Measurement::Angle(AngleUnit::Degree), value);
        assert_eq!(converted_fail, None);
    }

    #[test]
    fn test_angle_conversion() {
        let m = Measurement::Angle(AngleUnit::Degree);
        let value: f32 = 30.0;
        let converted = m.convert(&Measurement::Angle(AngleUnit::Degree), value);
        assert_ne!(converted, None);
        assert_eq!(converted.unwrap(), value);

        let converted_fail = m.convert(
            &Measurement::AngularVelocity(AngularVelocityUnit::RevolutionPerMinute),
            value,
        );
        assert_eq!(converted_fail, None);
    }

    #[test]
    fn test_length_conversion() {
        {
            let m = Measurement::Length(LengthUnit::Kilometer);
            let value_km: f32 = 1.5;
            let value_sm: f32 = 0.9320568;
            let value_nm: f32 = 0.8099352;

            let converted_km = m.convert(&Measurement::Length(LengthUnit::Kilometer), value_km);
            assert_ne!(converted_km, None);
            assert!(approx_eq!(
                f32,
                converted_km.unwrap(),
                value_km,
                F32Margin {
                    epsilon: 0.00001,
                    ulps: 2
                }
            ));

            let converted_sm = m.convert(&Measurement::Length(LengthUnit::StatuteMile), value_km);
            assert_ne!(converted_sm, None);
            assert!(approx_eq!(
                f32,
                converted_sm.unwrap(),
                value_sm,
                F32Margin {
                    epsilon: 0.00001,
                    ulps: 2
                }
            ));

            let converted_nm = m.convert(&Measurement::Length(LengthUnit::NauticalMile), value_km);
            assert_ne!(converted_nm, None);
            assert!(approx_eq!(
                f32,
                converted_nm.unwrap(),
                value_nm,
                F32Margin {
                    epsilon: 0.00001,
                    ulps: 2
                }
            ));
        }
        {
            let m = Measurement::Length(LengthUnit::StatuteMile);
            let value_sm: f32 = 3.5;
            let value_km: f32 = 5.632704;
            let value_nm: f32 = 3.041417;

            let converted_km = m.convert(&Measurement::Length(LengthUnit::Kilometer), value_sm);
            assert_ne!(converted_km, None);
            assert!(approx_eq!(
                f32,
                converted_km.unwrap(),
                value_km,
                F32Margin {
                    epsilon: 0.00001,
                    ulps: 2
                }
            ));

            let converted_sm = m.convert(&Measurement::Length(LengthUnit::StatuteMile), value_sm);
            assert_ne!(converted_sm, None);
            assert!(approx_eq!(
                f32,
                converted_sm.unwrap(),
                value_sm,
                F32Margin {
                    epsilon: 0.00001,
                    ulps: 2
                }
            ));

            let converted_nm = m.convert(&Measurement::Length(LengthUnit::NauticalMile), value_sm);
            assert_ne!(converted_nm, None);
            assert!(approx_eq!(
                f32,
                converted_nm.unwrap(),
                value_nm,
                F32Margin {
                    epsilon: 0.00001,
                    ulps: 2
                }
            ));
        }
    }
}
