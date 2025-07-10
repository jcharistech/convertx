//! # convertx
//!
//! A simple unit-conversion CLI supporting many unit types such as
//! + bytes 
//! + time
//! + length
//! + temperature
//! + mass 
//! + data rate, 
//! + area, volume, 
//! + speed
//! + pressure 
//! + electric current (ampere, milliampere)
//! + energy/work/heat (joule, calorie, kilowatt-hour, etc.)
//! + power (watt, kilowatt, horsepower)
//! + frequency (hertz, kilohertz)
//! + angle (degree, radian, gradian)
//! + force (newton, pound-force)
//! + luminous intensity (candela, lumen, lux)
//! + magnetic field (tesla, gauss)
//! + radioactivity (becquerel, curie)
//! + capacitance (farad)
//! + inductance (henry)
//! + conductance (siemens)
//! + electric charge (coulomb)
//! + voltage (volt)
//! + resistance (ohm)
//! + illuminance (lux, foot-candle)
//! + amount of substance (mole)
//!
//! ## Installation
//! Simply put the following in your **Cargo.toml**.
//! 
//! ```toml
//! [dependencies]
//! convertx = "0.1.0"
//! ```
//! Or use `cargo add convertx`
//! 
//! ## Usage
//!
//! ```sh
//! convertx <SUBCOMMAND> [OPTIONS]
//! ```
//!
//! ### Examples
//!
//! Convert 1024 bytes to megabytes:
//! ```sh
//! convertx bytes 1024 --megabytes
//! # Output: 1024 bytes = 0.00 MB
//! 
//! convertx bytes 1024 -m
//! # Output: 1024 bytes = 0.00 MB
//! ```
//!
//! Convert 3600 seconds to human-readable time:
//! ```sh
//! convertx time 3600 --human-readable
//! # Output: 3600 seconds = 1h 0m 0s
//! 
//! convertx time 3600 -h
//! # Output: 3600 seconds = 1h 0m 0s
//! ```
//!
//! Convert 1 kilometer to feet:
//! ```sh
//! convertx length 1 --from kilometers --to feet
//! # Output: 1.0000 kilometers = 3280.8400 feet
//! 
//! convertx length 10 -f kilometers -t feet
//! # Output: 10.0000 kilometers = 32800.8400 feet
//! ```
//!
//! Convert 100 Fahrenheit to Celsius:
//! ```sh
//! convertx temperature 100 --from F --to C
//! # Output: 100.00°F = 37.78°C
//! ```
//! Convert 1000 joules to kilowatt-hours:
//! ```sh
//! convertx energy 1000 --from joule --to kwh
//! # Output: 1000.00 joule = 0.00028 kWh
//! ```
//! 
//! Convert 1 kilowatt to horsepower:
//! ```sh
//! convertx power 1 --from kilowatt --to horsepower
//! # Output: 1.00 kW = 1.34102 hp
//! ```
//!
//! Run with `--help` to see all supported subcommands and options.
//!
use std::fmt;
use structopt::StructOpt;

/// Constant: Number of feet in a meter.
const FEET_IN_METER: f64 = 3.28084;
/// Constant: Number of inches in a meter.
const INCHES_IN_METER: f64 = 39.3701;
/// Constant: Number of kilograms in one pound.
const KG_IN_LB: f64 = 2.20462;
/// Constant: Number of ounces in one kilogram.
const OZ_IN_KG: f64 = 35.274;
/// Constant: Number of bits per second in one megabit per second.
const BPS_IN_MBPS: f64 = 1_000_000.0;
/// Constant: Zero-offset for Kelvin scale.
const KELVIN_OFFSET: f64 = 273.15;

/// Command-line interface definition for convertx.
/// Use `convertx <SUBCOMMAND> [OPTIONS]` for usage.
#[derive(StructOpt, Debug)]
#[structopt(name = "convertx", about = "Multi-purpose unit converter CLI")]
enum Cli {
    /// Convert byte values (e.g., bytes to MB or human readable).
    Bytes {
        /// Number of bytes to convert.
        num: u64,
        /// Convert bytes to megabytes.
        #[structopt(short, long)]
        megabytes: bool,
        /// Convert bytes to a human-readable string (e.g., "1.00 MB").
        #[structopt(short = "h", long = "human-readable")]
        human_readable: bool,
    },
    /// Convert time (seconds) to a human-readable format.
    Time {
        /// Seconds to convert.
        seconds: u64,
        /// Convert to human-readable format (e.g., "1h 13m 5s")
        #[structopt(short = "h", long = "human-readable")]
        human_readable: bool,
    },
    /// Convert length units.
    Length {
        /// Value to convert.
        value: f64,
        /// Unit to convert from (default: meters).
        #[structopt(short = "f", long, default_value = "meters", possible_values = &LengthUnit::variants(), case_insensitive = true)]
        from: LengthUnit,
        /// Unit to convert to (default: feet).
        #[structopt(short = "t", long, default_value = "feet", possible_values = &LengthUnit::variants(), case_insensitive = true)]
        to: LengthUnit,
    },
    /// Convert temperature units.
    Temperature {
        /// Value to convert.
        value: f64,
        /// Source temperature unit.
        #[structopt(short = "f", long, possible_values = &TempUnit::variants(), case_insensitive = true)]
        from: TempUnit,
        /// Target temperature unit.
        #[structopt(short = "t", long, possible_values = &TempUnit::variants(), case_insensitive = true)]
        to: TempUnit,
    },
    /// Convert mass/weight units.
    Mass {
        /// Value to convert.
        value: f64,
        /// Source mass unit.
        #[structopt(short = "f", long, possible_values = &MassUnit::variants(), case_insensitive = true)]
        from: MassUnit,
        /// Target mass unit.
        #[structopt(short = "t", long, possible_values = &MassUnit::variants(), case_insensitive = true)]
        to: MassUnit,
    },
    /// Convert data rate units.
    Datarate {
        /// Value to convert.
        value: f64,
        /// Source data rate unit.
        #[structopt(short = "f", long, possible_values = &DataRateUnit::variants(), case_insensitive = true)]
        from: DataRateUnit,
        /// Target data rate unit.
        #[structopt(short = "t", long, possible_values = &DataRateUnit::variants(), case_insensitive = true)]
        to: DataRateUnit,
    },
    /// Convert area units.
    Area {
        /// Value to convert.
        value: f64,
        /// Source area unit.
        #[structopt(short = "f", long, possible_values = &AreaUnit::variants(), case_insensitive = true)]
        from: AreaUnit,
        /// Target area unit.
        #[structopt(short = "t", long, possible_values = &AreaUnit::variants(), case_insensitive = true)]
        to: AreaUnit,
    },
    /// Convert volume units.
    Volume {
        /// Value to convert.
        value: f64,
        /// Source volume unit.
        #[structopt(short = "f", long, possible_values = &VolumeUnit::variants(), case_insensitive = true)]
        from: VolumeUnit,
        /// Target volume unit.
        #[structopt(short = "t", long, possible_values = &VolumeUnit::variants(), case_insensitive = true)]
        to: VolumeUnit,
    },
    /// Convert speed units.
    Speed {
        /// Value to convert.
        value: f64,
        /// Source speed unit.
        #[structopt(short = "f", long, possible_values = &SpeedUnit::variants(), case_insensitive = true)]
        from: SpeedUnit,
        /// Target speed unit.
        #[structopt(short = "t", long, possible_values = &SpeedUnit::variants(), case_insensitive = true)]
        to: SpeedUnit,
    },
    /// Convert pressure units.
    Pressure {
        /// Value to convert.
        value: f64,
        /// Source pressure unit.
        #[structopt(short = "f", long, possible_values = &PressureUnit::variants(), case_insensitive = true)]
        from: PressureUnit,
        /// Target pressure unit.
        #[structopt(short = "t", long, possible_values = &PressureUnit::variants(), case_insensitive = true)]
        to: PressureUnit,
    },
    /// Convert electric current units.
    Current {
        value: f64,
        #[structopt(short = "f", long, possible_values = &CurrentUnit::variants(), case_insensitive = true)]
        from: CurrentUnit,
        #[structopt(short = "t", long, possible_values = &CurrentUnit::variants(), case_insensitive = true)]
        to: CurrentUnit,
    },

    /// Convert energy/work/heat units.
    Energy {
        value: f64,
        #[structopt(short = "f", long, possible_values = &EnergyUnit::variants(), case_insensitive = true)]
        from: EnergyUnit,
        #[structopt(short = "t", long, possible_values = &EnergyUnit::variants(), case_insensitive = true)]
        to: EnergyUnit,
    },

    /// Convert power units.
    Power {
        value: f64,
        #[structopt(short = "f", long, possible_values = &PowerUnit::variants(), case_insensitive = true)]
        from: PowerUnit,
        #[structopt(short = "t", long, possible_values = &PowerUnit::variants(), case_insensitive = true)]
        to: PowerUnit,
    },

    /// Convert frequency units.
    Frequency {
        value: f64,
        #[structopt(short = "f", long, possible_values = &FrequencyUnit::variants(), case_insensitive = true)]
        from: FrequencyUnit,
        #[structopt(short = "t", long, possible_values = &FrequencyUnit::variants(), case_insensitive = true)]
        to: FrequencyUnit,
    },

    /// Convert angle units.
    Angle {
        value: f64,
        #[structopt(short = "f", long, possible_values = &AngleUnit::variants(), case_insensitive = true)]
        from: AngleUnit,
        #[structopt(short = "t", long, possible_values = &AngleUnit::variants(), case_insensitive = true)]
        to: AngleUnit,
    },

    /// Convert force units.
    Force {
        value: f64,
        #[structopt(short = "f", long, possible_values = &ForceUnit::variants(), case_insensitive = true)]
        from: ForceUnit,
        #[structopt(short = "t", long, possible_values = &ForceUnit::variants(), case_insensitive = true)]
        to: ForceUnit,
    },

    /// Convert luminous intensity/flux/illuminance units.
    Luminous {
        value: f64,
        #[structopt(short = "f", long, possible_values = &LuminousUnit::variants(), case_insensitive = true)]
        from: LuminousUnit,
        #[structopt(short = "t", long, possible_values = &LuminousUnit::variants(), case_insensitive = true)]
        to: LuminousUnit,
    },

    /// Convert magnetic field units.
    Magnetic {
        value: f64,
        #[structopt(short = "f", long, possible_values = &MagneticUnit::variants(), case_insensitive = true)]
        from: MagneticUnit,
        #[structopt(short = "t", long, possible_values = &MagneticUnit::variants(), case_insensitive = true)]
        to: MagneticUnit,
    },

    /// Convert radioactivity units.
    Radioactivity {
        value: f64,
        #[structopt(short = "f", long, possible_values = &RadioactivityUnit::variants(), case_insensitive = true)]
        from: RadioactivityUnit,
        #[structopt(short = "t", long, possible_values = &RadioactivityUnit::variants(), case_insensitive = true)]
        to: RadioactivityUnit,
    },

    /// Convert capacitance units.
    Capacitance {
        value: f64,
        #[structopt(short = "f", long, possible_values = &CapacitanceUnit::variants(), case_insensitive = true)]
        from: CapacitanceUnit,
        #[structopt(short = "t", long, possible_values = &CapacitanceUnit::variants(), case_insensitive = true)]
        to: CapacitanceUnit,
    },
        /// Convert inductance units.
    Inductance {
        value: f64,
        #[structopt(short = "f", long, possible_values = &InductanceUnit::variants(), case_insensitive = true)]
        from: InductanceUnit,
        #[structopt(short = "t", long, possible_values = &InductanceUnit::variants(), case_insensitive = true)]
        to: InductanceUnit,
    },

    /// Convert conductance units.
    Conductance {
        value: f64,
        #[structopt(short = "f", long, possible_values = &ConductanceUnit::variants(), case_insensitive = true)]
        from: ConductanceUnit,
        #[structopt(short = "t", long, possible_values = &ConductanceUnit::variants(), case_insensitive = true)]
        to: ConductanceUnit,
    },

    /// Convert electric charge units.
    Charge {
        value: f64,
        #[structopt(short = "f", long, possible_values = &ChargeUnit::variants(), case_insensitive = true)]
        from: ChargeUnit,
        #[structopt(short = "t", long, possible_values = &ChargeUnit::variants(), case_insensitive = true)]
        to: ChargeUnit,
    },

    /// Convert voltage units.
    Voltage {
        value: f64,
        #[structopt(short = "f", long, possible_values = &VoltageUnit::variants(), case_insensitive = true)]
        from: VoltageUnit,
        #[structopt(short = "t", long, possible_values = &VoltageUnit::variants(), case_insensitive = true)]
        to: VoltageUnit,
    },

    /// Convert resistance units.
    Resistance {
        value: f64,
        #[structopt(short = "f", long, possible_values = &ResistanceUnit::variants(), case_insensitive = true)]
        from: ResistanceUnit,
        #[structopt(short = "t", long, possible_values = &ResistanceUnit::variants(), case_insensitive = true)]
        to: ResistanceUnit,
    },

    /// Convert illuminance units.
    Illuminance {
        value: f64,
        #[structopt(short = "f", long, possible_values = &IlluminanceUnit::variants(), case_insensitive = true)]
        from: IlluminanceUnit,
        #[structopt(short = "t", long, possible_values = &IlluminanceUnit::variants(), case_insensitive = true)]
        to: IlluminanceUnit,
    },

    /// Convert amount of substance units.
    Amount {
        value: f64,
        #[structopt(short = "f", long, possible_values = &AmountUnit::variants(), case_insensitive = true)]
        from: AmountUnit,
        #[structopt(short = "t", long, possible_values = &AmountUnit::variants(), case_insensitive = true)]
        to: AmountUnit,
    },

}

/// Macro for quickly defining enums with string variants and utility implementations.
///
/// # Example
///
/// ```rust
/// enum_with_variants!(TempUnit {
///     C => "C",
///     F => "F",
///     K => "K",
/// });
/// ```
macro_rules! enum_with_variants {
    ($name:ident { $($variant:ident => $val:expr),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        enum $name {
            $($variant,)*
        }
        impl $name {
            /// Returns a static list of all variant names as strings.
            fn variants() -> &'static [&'static str] {
                &[$($val),*]
            }
        }
        impl ::std::str::FromStr for $name {
            type Err = String;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s.to_ascii_lowercase().as_str() {
                    $($val => Ok($name::$variant),)*
                    _ => Err(format!("invalid variant")),
                }
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                let s = match self {
                    $(Self::$variant => $val,)*
                };
                write!(f, "{}", s)
            }
        }
    }
}

// Define enums for each category with macro.
// Supported units for length.
enum_with_variants!(LengthUnit {
    Meters => "meters",
    Feet => "feet",
    Inches => "inches",
    Kilometers => "kilometers",
});

// Supported units for temperature.
enum_with_variants!(TempUnit {
    C => "c",
    F => "f",
    K => "k",
});

// Supported units for mass/weight.
enum_with_variants!(MassUnit {
    Kg => "kg",
    Lb => "lb",
    Oz => "oz",
});

// Supported units for data rate.
enum_with_variants!(DataRateUnit {
    Bps => "bps",
    Mbps => "mbps",
});

// Supported units for area.
enum_with_variants!(AreaUnit {
    SquareMeters => "sqm",
    SquareFeet => "sqft",
    Acres => "acres",
    Hectares => "hectares",
});

// Supported units for volume.
enum_with_variants!(VolumeUnit {
    Liters => "liters",
    Milliliters => "milliliters",
    CubicMeters => "cubic_meters",
    CubicInches => "cubic_inches",
    Gallons => "gallons",
});

// Supported units for speed.
enum_with_variants!(SpeedUnit {
    Mps => "mps",
    Kph => "kph",
    Mph => "mph",
    Knots => "knots",
});

// Supported units for pressure.
enum_with_variants!(PressureUnit {
    Pascal => "pa",
    Bar => "bar",
    Atm => "atm",
    Psi => "psi",
});


enum_with_variants!(CurrentUnit {
    Ampere => "ampere",
    Milliampere => "milliampere",
});

enum_with_variants!(EnergyUnit {
    Joule => "joule",
    Calorie => "calorie",
    Kwh => "kwh",
});

enum_with_variants!(PowerUnit {
    Watt => "watt",
    Kilowatt => "kilowatt",
    Horsepower => "horsepower",
});

enum_with_variants!(FrequencyUnit {
    Hertz => "hertz",
    Kilohertz => "kilohertz",
});

enum_with_variants!(AngleUnit {
    Degree => "degree",
    Radian => "radian",
    Gradian => "gradian",
});

enum_with_variants!(ForceUnit {
    Newton => "newton",
    PoundForce => "pound_force",
});

enum_with_variants!(LuminousUnit {
    Candela => "candela",
    Lumen => "lumen",
    Lux => "lux",
});

enum_with_variants!(MagneticUnit {
    Tesla => "tesla",
    Gauss => "gauss",
});

enum_with_variants!(RadioactivityUnit {
    Becquerel => "becquerel",
    Curie => "curie",
});

enum_with_variants!(CapacitanceUnit {
    Farad => "farad",
});

enum_with_variants!(InductanceUnit {
    Henry => "henry",
});

enum_with_variants!(ConductanceUnit {
    Siemens => "siemens",
});

enum_with_variants!(ChargeUnit {
    Coulomb => "coulomb",
});

enum_with_variants!(VoltageUnit {
    Volt => "volt",
});

enum_with_variants!(ResistanceUnit {
    Ohm => "ohm",
});

enum_with_variants!(IlluminanceUnit {
    Lux => "lux",
    FootCandle => "foot_candle",
});

enum_with_variants!(AmountUnit {
    Mole => "mole",
});

/// Convert bytes to megabytes.
///
/// # Example
/// ```
/// assert_eq!(bytes_to_mb(1048576), 1.0);
/// ```
fn bytes_to_mb(num_bytes: u64) -> f64 {
    num_bytes as f64 / (1024.0 * 1024.0)
}

/// Convert a number of bytes to a human-readable string.
///
/// # Example
/// ```
/// assert_eq!(bytes_to_human_readable(1048576), "1.00 MB");
/// ```
fn bytes_to_human_readable(num_bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut idx = 0;
    let mut n = num_bytes as f64;
    while n >= 1024.0 && idx < units.len() - 1 {
        n /= 1024.0;
        idx += 1;
    }
    format!("{:.2} {}", n, units[idx])
}

/// Convert seconds to a human-readable string (e.g., days, hours, minutes, seconds).
///
/// # Example
/// ```
/// assert_eq!(seconds_to_human_readable(3661), "1h 1m 1s");
/// ```
fn seconds_to_human_readable(seconds: u64) -> String {
    let (d, h, mut m, s);
    m = seconds / 60;
    s = seconds % 60;
    h = m / 60;
    m = m % 60;
    d = h / 24;
    let mut parts = vec![];
    if d > 0 {
        parts.push(format!("{}d", d));
    }
    if h % 24 > 0 {
        parts.push(format!("{}h", h % 24));
    }
    if m > 0 {
        parts.push(format!("{}m", m));
    }
    if s > 0 || parts.is_empty() {
        parts.push(format!("{}s", s));
    }
    parts.join(" ")
}

/// Convert between length units.
///
/// Returns `Some(result)` if conversion is supported.
///
/// # Example
/// ```
/// use crate::LengthUnit::*;
/// assert!((convert_length(1.0, Meters, Feet).unwrap() - 3.28084).abs() < 1e-5);
/// ```
fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> Option<f64> {
    use LengthUnit::*;
    let in_meters = match from {
        Meters => value,
        Feet => value / FEET_IN_METER,
        Inches => value / INCHES_IN_METER,
        Kilometers => value * 1000.0,
    };
    let result = match to {
        Meters => in_meters,
        Feet => in_meters * FEET_IN_METER,
        Inches => in_meters * INCHES_IN_METER,
        Kilometers => in_meters / 1000.0,
    };
    Some(result)
}

/// Convert between temperature units (Celsius, Fahrenheit, Kelvin).
///
/// # Example
/// ```
/// use crate::TempUnit::*;
/// assert!((convert_temp(0.0, C, F).unwrap() - 32.0).abs() < 1e-6);
/// ```
fn convert_temp(value: f64, from: TempUnit, to: TempUnit) -> Option<f64> {
    use TempUnit::*;
    let celsius = match from {
        C => value,
        F => (value - 32.0) * 5.0 / 9.0,
        K => value - KELVIN_OFFSET,
    };
    let result = match to {
        C => celsius,
        F => celsius * 9.0 / 5.0 + 32.0,
        K => celsius + KELVIN_OFFSET,
    };
    Some(result)
}

/// Convert between mass units.
///
/// # Example
/// ```
/// use crate::MassUnit::*;
/// assert!((convert_mass(1.0, Kg, Lb).unwrap() - 2.20462).abs() < 1e-5);
/// ```
fn convert_mass(value: f64, from: MassUnit, to: MassUnit) -> Option<f64> {
    use MassUnit::*;
    let in_kg = match from {
        Kg => value,
        Lb => value / KG_IN_LB,
        Oz => value / OZ_IN_KG,
    };
    let result = match to {
        Kg => in_kg,
        Lb => in_kg * KG_IN_LB,
        Oz => in_kg * OZ_IN_KG,
    };
    Some(result)
}

/// Convert between data rate units (bps, Mbps).
///
/// # Example
/// ```
/// use crate::DataRateUnit::*;
/// assert_eq!(convert_datarate(1_000_000.0, Bps, Mbps), Some(1.0));
/// ```
fn convert_datarate(value: f64, from: DataRateUnit, to: DataRateUnit) -> Option<f64> {
    use DataRateUnit::*;
    match (from, to) {
        (Bps, Mbps) => Some(value / BPS_IN_MBPS),
        (Mbps, Bps) => Some(value * BPS_IN_MBPS),
        _ => Some(value),
    }
}

/// Convert between area units.
///
/// # Example
/// ```
/// use crate::AreaUnit::*;
/// assert!((convert_area(1.0, Acres, SquareMeters).unwrap() - 4046.85642).abs() < 1e-4);
/// ```
fn convert_area(value: f64, from: AreaUnit, to: AreaUnit) -> Option<f64> {
    use AreaUnit::*;
    let sqm = match from {
        SquareMeters => value,
        SquareFeet => value / 10.7639,
        Acres => value * 4046.85642,
        Hectares => value * 10000.0,
    };
    let result = match to {
        SquareMeters => sqm,
        SquareFeet => sqm * 10.7639,
        Acres => sqm / 4046.85642,
        Hectares => sqm / 10000.0,
    };
    Some(result)
}

/// Convert between volume units.
///
/// # Example
/// ```
/// use crate::VolumeUnit::*;
/// assert!((convert_volume(1.0, Gallons, Liters).unwrap() - 3.78541).abs() < 1e-5);
/// ```
fn convert_volume(value: f64, from: VolumeUnit, to: VolumeUnit) -> Option<f64> {
    use VolumeUnit::*;
    let liters = match from {
        Liters => value,
        Milliliters => value / 1000.0,
        CubicMeters => value * 1000.0,
        CubicInches => value / 61.0237,
        Gallons => value * 3.78541,
    };
    let result = match to {
        Liters => liters,
        Milliliters => liters * 1000.0,
        CubicMeters => liters / 1000.0,
        CubicInches => liters * 61.0237,
        Gallons => liters / 3.78541,
    };
    Some(result)
}

/// Convert between speed units.
///
/// # Example
/// ```
/// use crate::SpeedUnit::*;
/// assert!((convert_speed(1.0, Mps, Kph).unwrap() - 3.6).abs() < 1e-6);
/// ```
fn convert_speed(value: f64, from: SpeedUnit, to: SpeedUnit) -> Option<f64> {
    use SpeedUnit::*;
    let mps = match from {
        Mps => value,
        Kph => value / 3.6,
        Mph => value * 0.44704,
        Knots => value * 0.514444,
    };
    let result = match to {
        Mps => mps,
        Kph => mps * 3.6,
        Mph => mps / 0.44704,
        Knots => mps / 0.514444,
    };
    Some(result)
}

/// Convert between pressure units.
///
/// # Example
/// ```
/// use crate::PressureUnit::*;
/// assert!((convert_pressure(1.0, Atm, Pascal).unwrap() - 101325.0).abs() < 1e-3);
/// ```
fn convert_pressure(value: f64, from: PressureUnit, to: PressureUnit) -> Option<f64> {
    use PressureUnit::*;
    let pa = match from {
        Pascal => value,
        Bar => value * 100000.0,
        Atm => value * 101325.0,
        Psi => value * 6894.76,
    };
    let result = match to {
        Pascal => pa,
        Bar => pa / 100000.0,
        Atm => pa / 101325.0,
        Psi => pa / 6894.76,
    };
    Some(result)
}

/// Convert between electric current units (ampere, milliampere).
///
/// # Examples
/// ```
/// assert_eq!(convert_current(2.0, CurrentUnit::Ampere, CurrentUnit::Milliampere), Some(2000.0));
/// assert_eq!(convert_current(4000.0, CurrentUnit::Milliampere, CurrentUnit::Ampere), Some(4.0));
/// ```
fn convert_current(value: f64, from: CurrentUnit, to: CurrentUnit) -> Option<f64> {
    use CurrentUnit::*;
    let amp = match from {
        Ampere => value,
        Milliampere => value / 1000.0,
    };
    let result = match to {
        Ampere => amp,
        Milliampere => amp * 1000.0,
    };
    Some(result)
}

/// Convert between energy, work, and heat units (joule, calorie, kWh).
///
/// # Examples
/// ```
/// assert!((convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Calorie).unwrap() - 0.239006).abs() < 1e-6);
/// ```
fn convert_energy(value: f64, from: EnergyUnit, to: EnergyUnit) -> Option<f64> {
    use EnergyUnit::*;
    let joule = match from {
        Joule => value,
        Calorie => value * 4.184,
        Kwh => value * 3_600_000.0,
    };
    let result = match to {
        Joule => joule,
        Calorie => joule / 4.184,
        Kwh => joule / 3_600_000.0,
    };
    Some(result)
}

/// Convert between power units (watt, kilowatt, horsepower).
///
/// # Examples
/// ```
/// assert!((convert_power(1.0, PowerUnit::Kilowatt, PowerUnit::Horsepower).unwrap() - 1.34102).abs() < 1e-5);
/// ```
fn convert_power(value: f64, from: PowerUnit, to: PowerUnit) -> Option<f64> {
    use PowerUnit::*;
    let watt = match from {
        Watt => value,
        Kilowatt => value * 1000.0,
        Horsepower => value * 745.699872,
    };
    let result = match to {
        Watt => watt,
        Kilowatt => watt / 1000.0,
        Horsepower => watt / 745.699872,
    };
    Some(result)
}

/// Convert between frequency units (hertz, kilohertz).
///
/// # Examples
/// ```
/// assert_eq!(convert_frequency(1500.0, FrequencyUnit::Hertz, FrequencyUnit::Kilohertz), Some(1.5));
/// ```
fn convert_frequency(value: f64, from: FrequencyUnit, to: FrequencyUnit) -> Option<f64> {
    use FrequencyUnit::*;
    let hz = match from {
        Hertz => value,
        Kilohertz => value * 1000.0,
    };
    let result = match to {
        Hertz => hz,
        Kilohertz => hz / 1000.0,
    };
    Some(result)
}

/// Convert between angle units (degree, radian, gradian).
///
/// # Examples
/// ```
/// assert!((convert_angle(180.0, AngleUnit::Degree, AngleUnit::Radian).unwrap() - std::f64::consts::PI).abs() < 1e-10);
/// ```
fn convert_angle(value: f64, from: AngleUnit, to: AngleUnit) -> Option<f64> {
    use AngleUnit::*;
    let in_deg = match from {
        Degree => value,
        Radian => value * 180.0 / std::f64::consts::PI,
        Gradian => value * 0.9,
    };
    let result = match to {
        Degree => in_deg,
        Radian => in_deg * std::f64::consts::PI / 180.0,
        Gradian => in_deg / 0.9,
    };
    Some(result)
}

/// Convert between force units (newton, pound-force).
///
/// # Examples
/// ```
/// assert!((convert_force(10.0, ForceUnit::Newton, ForceUnit::PoundForce).unwrap() - 2.24809).abs() < 1e-5);
/// ```
fn convert_force(value: f64, from: ForceUnit, to: ForceUnit) -> Option<f64> {
    use ForceUnit::*;
    let newton = match from {
        Newton => value,
        PoundForce => value * 4.4482216153,
    };
    let result = match to {
        Newton => newton,
        PoundForce => newton / 4.4482216153,
    };
    Some(result)
}

/// Convert between luminous units (candela, lumen, lux).
///
/// **Note:** These units are not generally directly interconvertible as they represent different physical quantities.
/// This function only supports identity conversions.
///
/// # Examples
/// ```
/// assert_eq!(convert_luminous(5.0, LuminousUnit::Candela, LuminousUnit::Candela), Some(5.0));
/// ```
fn convert_luminous(value: f64, from: LuminousUnit, to: LuminousUnit) -> Option<f64> {
    use LuminousUnit::*;
    match (from, to) {
        (Candela, Candela) | (Lumen, Lumen) | (Lux, Lux) => Some(value),
        _ => None,
    }
}

/// Convert between magnetic field units (tesla, gauss).
///
/// # Examples
/// ```
/// assert_eq!(convert_magnetic(1.0, MagneticUnit::Tesla, MagneticUnit::Gauss), Some(10000.0));
/// ```
fn convert_magnetic(value: f64, from: MagneticUnit, to: MagneticUnit) -> Option<f64> {
    use MagneticUnit::*;
    let tesla = match from {
        Tesla => value,
        Gauss => value / 10_000.0,
    };
    let result = match to {
        Tesla => tesla,
        Gauss => tesla * 10_000.0,
    };
    Some(result)
}

/// Convert between radioactivity units (becquerel, curie).
///
/// # Examples
/// ```
/// assert!((convert_radioactivity(1.0, RadioactivityUnit::Curie, RadioactivityUnit::Becquerel).unwrap() - 3.7e10).abs() < 1e2);
/// assert!((convert_radioactivity(3.7e10, RadioactivityUnit::Becquerel, RadioactivityUnit::Curie).unwrap() - 1.0).abs() < 1e-10);
/// ```
fn convert_radioactivity(value: f64, from: RadioactivityUnit, to: RadioactivityUnit) -> Option<f64> {
    use RadioactivityUnit::*;
    let becquerel = match from {
        Becquerel => value,
        Curie => value * 3.7e10,
    };
    let result = match to {
        Becquerel => becquerel,
        Curie => becquerel / 3.7e10,
    };
    Some(result)
}

/// Convert capacitance units (farad only).
///
/// # Examples
/// ```
/// assert_eq!(convert_capacitance(1.0, CapacitanceUnit::Farad, CapacitanceUnit::Farad), Some(1.0));
/// ```
fn convert_capacitance(value: f64, _from: CapacitanceUnit, _to: CapacitanceUnit) -> Option<f64> {
    Some(value)
}

/// Convert inductance units (henry only).
///
/// # Examples
/// ```
/// assert_eq!(convert_inductance(1.0, InductanceUnit::Henry, InductanceUnit::Henry), Some(1.0));
/// ```
fn convert_inductance(value: f64, _from: InductanceUnit, _to: InductanceUnit) -> Option<f64> {
    Some(value)
}

/// Convert conductance units (siemens only).
///
/// # Examples
/// ```
/// assert_eq!(convert_conductance(1.0, ConductanceUnit::Siemens, ConductanceUnit::Siemens), Some(1.0));
/// ```
fn convert_conductance(value: f64, _from: ConductanceUnit, _to: ConductanceUnit) -> Option<f64> {
    Some(value)
}

/// Convert electric charge units (coulomb only).
///
/// # Examples
/// ```
/// assert_eq!(convert_charge(4.0, ChargeUnit::Coulomb, ChargeUnit::Coulomb), Some(4.0));
/// ```
fn convert_charge(value: f64, _from: ChargeUnit, _to: ChargeUnit) -> Option<f64> {
    Some(value)
}

/// Convert voltage units (volt only).
///
/// # Examples
/// ```
/// assert_eq!(convert_voltage(12.0, VoltageUnit::Volt, VoltageUnit::Volt), Some(12.0));
/// ```
fn convert_voltage(value: f64, _from: VoltageUnit, _to: VoltageUnit) -> Option<f64> {
    Some(value)
}

/// Convert resistance units (ohm only).
///
/// # Examples
/// ```
/// assert_eq!(convert_resistance(100.0, ResistanceUnit::Ohm, ResistanceUnit::Ohm), Some(100.0));
/// ```
fn convert_resistance(value: f64, _from: ResistanceUnit, _to: ResistanceUnit) -> Option<f64> {
    Some(value)
}

/// Convert between illuminance units (lux, foot-candle).
///
/// # Examples
/// ```
/// assert!((convert_illuminance(1.0, IlluminanceUnit::FootCandle, IlluminanceUnit::Lux).unwrap() - 10.76391).abs() < 1e-5);
/// assert!((convert_illuminance(10.76391, IlluminanceUnit::Lux, IlluminanceUnit::FootCandle).unwrap() - 1.0).abs() < 1e-5);
/// ```
fn convert_illuminance(value: f64, from: IlluminanceUnit, to: IlluminanceUnit) -> Option<f64> {
    use IlluminanceUnit::*;
    let lux = match from {
        Lux => value,
        FootCandle => value * 10.76391,
    };
    let result = match to {
        Lux => lux,
        FootCandle => lux / 10.76391,
    };
    Some(result)
}

/// Convert amount of substance units (mole only).
///
/// # Examples
/// ```
/// assert_eq!(convert_amount(2.0, AmountUnit::Mole, AmountUnit::Mole), Some(2.0));
/// ```
fn convert_amount(value: f64, _from: AmountUnit, _to: AmountUnit) -> Option<f64> {
    Some(value)
}



/// Entry point for the CLI application.
///
/// Parses CLI arguments, dispatches the appropriate conversion, and prints results.

fn main() {
    let cli = Cli::from_args();
    match cli {
        Cli::Bytes {
            num,
            megabytes,
            human_readable,
        } => {
            if megabytes {
                println!("{} bytes = {:.2} MB", num, bytes_to_mb(num));
            } else if human_readable {
                println!("{} bytes = {}", num, bytes_to_human_readable(num));
            } else {
                println!("Please specify --megabytes or --human-readable. See --help.");
            }
        }
        Cli::Time {
            seconds,
            human_readable,
        } => {
            if human_readable {
                println!(
                    "{} seconds = {}",
                    seconds,
                    seconds_to_human_readable(seconds)
                );
            } else {
                println!("Please specify --human-readable. See --help.");
            }
        }
        Cli::Length { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_length(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Temperature { value, from, to } => {
            if from == to {
                println!(
                    "{:.2}°{} = {:.2}°{}",
                    value,
                    format!("{}", from).to_uppercase(),
                    to,
                    format!("{}", to).to_uppercase()
                );
            } else if let Some(result) = convert_temp(value, from.clone(), to.clone()) {
                println!(
                    "{:.2}°{} = {:.2}°{}",
                    value,
                    format!("{}", from).to_uppercase(),
                    result,
                    format!("{}", to).to_uppercase()
                );
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }

        Cli::Mass { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_mass(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Datarate { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_datarate(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Area { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_area(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Volume { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_volume(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Speed { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_speed(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Pressure { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_pressure(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
          Cli::Current { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_current(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Energy { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_energy(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Power { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_power(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Frequency { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_frequency(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Angle { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_angle(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Force { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_force(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Luminous { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_luminous(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!(
                    "Conversion from {} to {} is not directly supported (dimensional incompatibility).",
                    from, to
                );
            }
        }
        Cli::Magnetic { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_magnetic(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Radioactivity { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_radioactivity(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Capacitance { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_capacitance(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Inductance { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_inductance(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Conductance { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_conductance(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Charge { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_charge(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Voltage { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_voltage(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            }
        }    
     Cli::Resistance { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_resistance(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
    Cli::Illuminance { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_illuminance(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
        Cli::Amount { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_amount(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_mb() {
        assert_eq!(bytes_to_mb(1048576), 1.0);
        assert!((bytes_to_mb(2097152) - 2.0).abs() < 1e-8);
    }

    #[test]
    fn test_bytes_to_human_readable() {
        assert_eq!(bytes_to_human_readable(1023), "1023.00 B");
        assert_eq!(bytes_to_human_readable(1024), "1.00 KB");
        assert_eq!(bytes_to_human_readable(1048576), "1.00 MB");
    }

    #[test]
    fn test_seconds_to_human_readable() {
        assert_eq!(seconds_to_human_readable(59), "59s");
        assert_eq!(seconds_to_human_readable(61), "1m 1s");
        assert_eq!(seconds_to_human_readable(3661), "1h 1m 1s");
        assert_eq!(seconds_to_human_readable(90061), "1d 1h 1m 1s");
    }

    #[test]
    fn test_convert_length() {
        use LengthUnit::*;
        assert!((convert_length(1.0, Meters, Feet).unwrap() - 3.28084).abs() < 1e-5);
        assert!((convert_length(3.28084, Feet, Meters).unwrap() - 1.0).abs() < 1e-5);
        assert!((convert_length(1.0, Kilometers, Meters).unwrap() - 1000.0).abs() < 1e-5);
        assert!((convert_length(12.0, Inches, Feet).unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_convert_temp() {
        use TempUnit::*;
        assert!((convert_temp(0.0, C, F).unwrap() - 32.0).abs() < 1e-6);
        assert!((convert_temp(32.0, F, C).unwrap() - 0.0).abs() < 1e-6);
        assert!((convert_temp(100.0, C, K).unwrap() - 373.15).abs() < 1e-2);
        assert!((convert_temp(0.0, K, C).unwrap() - -273.15).abs() < 1e-2);
    }

    #[test]
    fn test_convert_mass() {
        use MassUnit::*;
        assert!((convert_mass(1.0, Kg, Lb).unwrap() - 2.20462).abs() < 1e-5);
        assert!((convert_mass(2.20462, Lb, Kg).unwrap() - 1.0).abs() < 1e-5);
        assert!((convert_mass(1.0, Kg, Oz).unwrap() - 35.274).abs() < 1e-3);
        assert!((convert_mass(35.274, Oz, Kg).unwrap() - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_convert_datarate() {
        use DataRateUnit::*;
        assert!((convert_datarate(1_000_000.0, Bps, Mbps).unwrap() - 1.0).abs() < 1e-8);
        assert!((convert_datarate(1.0, Mbps, Bps).unwrap() - 1_000_000.0).abs() < 1e-8);
    }

    #[test]
    fn test_convert_area() {
        use AreaUnit::*;
        assert!((convert_area(1.0, Acres, SquareMeters).unwrap() - 4046.85642).abs() < 1e-4);
        assert!((convert_area(1.0, SquareMeters, Acres).unwrap() - 0.000247105).abs() < 1e-7);
        assert!((convert_area(1.0, Hectares, Acres).unwrap() - 2.47105).abs() < 1e-5);
    }

    #[test]
    fn test_convert_volume() {
        use VolumeUnit::*;
        assert!((convert_volume(1.0, Gallons, Liters).unwrap() - 3.78541).abs() < 1e-5);
        assert!((convert_volume(1.0, Liters, Gallons).unwrap() - 0.264172).abs() < 1e-6);
        assert!((convert_volume(1000.0, Milliliters, Liters).unwrap() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_convert_speed() {
        use SpeedUnit::*;
        assert!((convert_speed(1.0, Mps, Kph).unwrap() - 3.6).abs() < 1e-6);
        assert!((convert_speed(3.6, Kph, Mps).unwrap() - 1.0).abs() < 1e-6);
        assert!((convert_speed(1.0, Knots, Mph).unwrap() - 1.15078).abs() < 1e-5);
    }

    #[test]
    fn test_convert_pressure() {
        use PressureUnit::*;
        assert!((convert_pressure(1.0, Atm, Pascal).unwrap() - 101325.0).abs() < 1e-3);
        assert!((convert_pressure(1.0, Psi, Bar).unwrap() - 0.0689476).abs() < 1e-6);
        assert!((convert_pressure(1.0, Bar, Psi).unwrap() - 14.5038).abs() < 1e-4);
    }

    #[test]
    fn test_convert_current() {
        assert_eq!(convert_current(1.0, CurrentUnit::Ampere, CurrentUnit::Milliampere), Some(1000.0));
        assert_eq!(convert_current(5000.0, CurrentUnit::Milliampere, CurrentUnit::Ampere), Some(5.0));
    }

    #[test]
    fn test_convert_energy() {
        assert!((convert_energy(1.0, EnergyUnit::Joule, EnergyUnit::Calorie).unwrap() - 0.2390057361).abs() < 1e-6);
        assert!((convert_energy(1.0, EnergyUnit::Calorie, EnergyUnit::Joule).unwrap() - 4.184).abs() < 1e-6);
        assert!((convert_energy(1.0, EnergyUnit::Kwh, EnergyUnit::Joule).unwrap() - 3_600_000.0).abs() < 1e-3);
    }

    #[test]
    fn test_convert_power() {
        assert!((convert_power(1.0, PowerUnit::Kilowatt, PowerUnit::Horsepower).unwrap() - 1.34102209).abs() < 1e-6);
        assert!((convert_power(10.0, PowerUnit::Horsepower, PowerUnit::Watt).unwrap() - 7456.99872).abs() < 1e-3);
    }

    #[test]
    fn test_convert_frequency() {
        assert_eq!(convert_frequency(1000.0, FrequencyUnit::Hertz, FrequencyUnit::Kilohertz), Some(1.0));
        assert_eq!(convert_frequency(2.5, FrequencyUnit::Kilohertz, FrequencyUnit::Hertz), Some(2500.0));
    }

    #[test]
    fn test_convert_angle() {
        use std::f64::consts::PI;
        assert!((convert_angle(180.0, AngleUnit::Degree, AngleUnit::Radian).unwrap() - PI).abs() < 1e-10);
        assert!((convert_angle(100.0, AngleUnit::Gradian, AngleUnit::Degree).unwrap() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_force() {
        assert!((convert_force(10.0, ForceUnit::Newton, ForceUnit::PoundForce).unwrap() - 2.248089).abs() < 1e-5);
        assert!((convert_force(1.0, ForceUnit::PoundForce, ForceUnit::Newton).unwrap() - 4.4482216153).abs() < 1e-9);
    }

    #[test]
    fn test_convert_luminous() {
        assert_eq!(convert_luminous(5.0, LuminousUnit::Candela, LuminousUnit::Candela), Some(5.0));
        assert_eq!(convert_luminous(10.0, LuminousUnit::Lux, LuminousUnit::Lux), Some(10.0));
        assert_eq!(convert_luminous(1.0, LuminousUnit::Candela, LuminousUnit::Lumen), None);
    }

    #[test]
    fn test_convert_magnetic() {
        assert_eq!(convert_magnetic(1.0, MagneticUnit::Tesla, MagneticUnit::Gauss), Some(10000.0));
        assert!((convert_magnetic(10000.0, MagneticUnit::Gauss, MagneticUnit::Tesla).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_radioactivity() {
        assert!((convert_radioactivity(1.0, RadioactivityUnit::Curie, RadioactivityUnit::Becquerel).unwrap() - 3.7e10).abs() < 1e2);
        assert!((convert_radioactivity(3.7e10, RadioactivityUnit::Becquerel, RadioactivityUnit::Curie).unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_capacitance() {
        assert_eq!(convert_capacitance(2.0, CapacitanceUnit::Farad, CapacitanceUnit::Farad), Some(2.0));
    }

    #[test]
    fn test_convert_inductance() {
        assert_eq!(convert_inductance(5.0, InductanceUnit::Henry, InductanceUnit::Henry), Some(5.0));
    }

    #[test]
    fn test_convert_conductance() {
        assert_eq!(convert_conductance(3.0, ConductanceUnit::Siemens, ConductanceUnit::Siemens), Some(3.0));
    }

    #[test]
    fn test_convert_charge() {
        assert_eq!(convert_charge(8.0, ChargeUnit::Coulomb, ChargeUnit::Coulomb), Some(8.0));
    }

    #[test]
    fn test_convert_voltage() {
        assert_eq!(convert_voltage(12.0, VoltageUnit::Volt, VoltageUnit::Volt), Some(12.0));
    }

    #[test]
    fn test_convert_resistance() {
        assert_eq!(convert_resistance(20.0, ResistanceUnit::Ohm, ResistanceUnit::Ohm), Some(20.0));
    }

    #[test]
    fn test_convert_illuminance() {
        assert!((convert_illuminance(1.0, IlluminanceUnit::FootCandle, IlluminanceUnit::Lux).unwrap() - 10.76391).abs() < 1e-5);
        assert!((convert_illuminance(10.76391, IlluminanceUnit::Lux, IlluminanceUnit::FootCandle).unwrap() - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_convert_amount() {
        assert_eq!(convert_amount(2.0, AmountUnit::Mole, AmountUnit::Mole), Some(2.0));
    }

}
