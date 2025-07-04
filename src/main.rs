use structopt::StructOpt;
use std::fmt;

const FEET_IN_METER: f64 = 3.28084;
const INCHES_IN_METER: f64 = 39.3701;
const KG_IN_LB: f64 = 2.20462;
const OZ_IN_KG: f64 = 35.274;
const BPS_IN_MBPS: f64 = 1_000_000.0;
const KELVIN_OFFSET: f64 = 273.15;

#[derive(StructOpt, Debug)]
#[structopt(name = "convertx", about = "Multi-purpose unit converter CLI")]
enum Cli {
    /// Convert byte values
    Bytes {
        /// Number of bytes
        num: u64,
        /// Convert bytes to megabytes
        #[structopt(short, long)]
        megabytes: bool,
        /// Convert bytes to human-readable
        #[structopt(short = "r", long = "human-readable")]
        human_readable: bool,
    },
    /// Convert time values in seconds
    Time {
        /// Seconds
        seconds: u64,
        /// Convert seconds to human-readable time
        #[structopt(short = "r", long = "human-readable")]
        human_readable: bool,
    },
    /// Convert length units
    Length {
        value: f64,
        #[structopt(short = "f", long, default_value = "meters", possible_values = &LengthUnit::variants(), case_insensitive = true)]
        from: LengthUnit,
        #[structopt(short = "t", long, default_value = "feet", possible_values = &LengthUnit::variants(), case_insensitive = true)]
        to: LengthUnit,
    },
    /// Convert temperature units
    Temperature {
        value: f64,
        #[structopt(short = "f", long, possible_values = &TempUnit::variants(), case_insensitive = true)]
        from: TempUnit,
        #[structopt(short = "t", long, possible_values = &TempUnit::variants(), case_insensitive = true)]
        to: TempUnit,
    },
    /// Convert mass/weight units
    Mass {
        value: f64,
        #[structopt(short = "f", long, possible_values = &MassUnit::variants(), case_insensitive = true)]
        from: MassUnit,
        #[structopt(short = "t", long, possible_values = &MassUnit::variants(), case_insensitive = true)]
        to: MassUnit,
    },
    /// Convert data rate units
    Datarate {
        value: f64,
        #[structopt(short = "f", long, possible_values = &DataRateUnit::variants(), case_insensitive = true)]
        from: DataRateUnit,
        #[structopt(short = "t", long, possible_values = &DataRateUnit::variants(), case_insensitive = true)]
        to: DataRateUnit,
    },

        /// Convert area units
    Area {
        value: f64,
        #[structopt(short = "f", long, possible_values = &AreaUnit::variants(), case_insensitive = true)]
        from: AreaUnit,
        #[structopt(short = "t", long, possible_values = &AreaUnit::variants(), case_insensitive = true)]
        to: AreaUnit,
    },
    /// Convert volume units
    Volume {
        value: f64,
        #[structopt(short = "f", long, possible_values = &VolumeUnit::variants(), case_insensitive = true)]
        from: VolumeUnit,
        #[structopt(short = "t", long, possible_values = &VolumeUnit::variants(), case_insensitive = true)]
        to: VolumeUnit,
    },
    /// Convert speed units
    Speed {
        value: f64,
        #[structopt(short = "f", long, possible_values = &SpeedUnit::variants(), case_insensitive = true)]
        from: SpeedUnit,
        #[structopt(short = "t", long, possible_values = &SpeedUnit::variants(), case_insensitive = true)]
        to: SpeedUnit,
    },
    /// Convert pressure units
    Pressure {
        value: f64,
        #[structopt(short = "f", long, possible_values = &PressureUnit::variants(), case_insensitive = true)]
        from: PressureUnit,
        #[structopt(short = "t", long, possible_values = &PressureUnit::variants(), case_insensitive = true)]
        to: PressureUnit,
    },

}

// Util traits for Arg enums
macro_rules! enum_with_variants {
    ($name:ident { $($variant:ident => $val:expr),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        enum $name {
            $($variant,)*
        }
        impl $name {
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

enum_with_variants!(LengthUnit {
    Meters => "meters",
    Feet => "feet",
    Inches => "inches",
    Kilometers => "kilometers",
});

enum_with_variants!(TempUnit {
    C => "C",
    F => "F",
    K => "K",
});

enum_with_variants!(MassUnit {
    Kg => "kg",
    Lb => "lb",
    Oz => "oz",
});

enum_with_variants!(DataRateUnit {
    Bps => "bps",
    Mbps => "mbps",
});

enum_with_variants!(AreaUnit {
    SquareMeters => "sqm",
    SquareFeet => "sqft",
    Acres => "acres",
    Hectares => "hectares",
});

enum_with_variants!(VolumeUnit {
    Liters => "liters",
    Milliliters => "milliliters",
    CubicMeters => "cubic_meters",
    CubicInches => "cubic_inches",
    Gallons => "gallons",
});

enum_with_variants!(SpeedUnit {
    Mps => "mps",
    Kph => "kph",
    Mph => "mph",
    Knots => "knots",
});

enum_with_variants!(PressureUnit {
    Pascal => "pa",
    Bar => "bar",
    Atm => "atm",
    Psi => "psi",
});


// Conversion functions 
fn bytes_to_mb(num_bytes: u64) -> f64 {
    num_bytes as f64 / (1024.0 * 1024.0)
}
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
fn seconds_to_human_readable(seconds: u64) -> String {
    let (d, h, mut m, s);
    m = seconds / 60;
    s = seconds % 60;
    h = m / 60;
    m = m % 60;
    d = h / 24;
    let mut parts = vec![];
    if d > 0 { parts.push(format!("{}d", d)); }
    if h % 24 > 0 { parts.push(format!("{}h", h % 24)); }
    if m > 0 { parts.push(format!("{}m", m)); }
    if s > 0 || parts.is_empty() { parts.push(format!("{}s", s)); }
    parts.join(" ")
}
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

fn convert_datarate(value: f64, from: DataRateUnit, to: DataRateUnit) -> Option<f64> {
    use DataRateUnit::*;
    match (from, to) {
        (Bps, Mbps) => Some(value / BPS_IN_MBPS),
        (Mbps, Bps) => Some(value * BPS_IN_MBPS),
        _ => Some(value),
    }
}

// --- Area Conversions ---
fn convert_area(value: f64, from: AreaUnit, to: AreaUnit) -> Option<f64> {
    use AreaUnit::*;
    // convert everything to square_meters first
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

// --- Volume Conversions ---
fn convert_volume(value: f64, from: VolumeUnit, to: VolumeUnit) -> Option<f64> {
    use VolumeUnit::*;
    // All to liters
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

// --- Speed Conversions ---
fn convert_speed(value: f64, from: SpeedUnit, to: SpeedUnit) -> Option<f64> {
    use SpeedUnit::*;
    // All to meters_per_second
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

// --- Pressure Conversions ---
fn convert_pressure(value: f64, from: PressureUnit, to: PressureUnit) -> Option<f64> {
    use PressureUnit::*;
    // All to pascal
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



fn main() {
    let cli = Cli::from_args();
    match cli {
        Cli::Bytes { num, megabytes, human_readable } => {
            if megabytes {
                println!("{} bytes = {:.2} MB", num, bytes_to_mb(num));
            } else if human_readable {
                println!("{} bytes = {}", num, bytes_to_human_readable(num));
            } else {
                println!("Please specify --megabytes or --human-readable. See --help.");
            }
        }
        Cli::Time { seconds, human_readable } => {
            if human_readable {
                println!("{} seconds = {}", seconds, seconds_to_human_readable(seconds));
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
                println!("{:.2}°{} = {:.2}°{}", value, format!("{}", from).to_uppercase(), to, format!("{}", to).to_uppercase());
            } else if let Some(result) = convert_temp(value, from.clone(), to.clone()) {
                println!("{:.2}°{} = {:.2}°{}", value, format!("{}", from).to_uppercase(), result, format!("{}", to).to_uppercase());
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
        },
        Cli::Speed { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_speed(value, from.clone(), to.clone()) {
                println!("{:.4} {} = {:.4} {}", value, from, result, to);
            } else {
                println!("Conversion from {} to {} not supported.", from, to);
            }
        },
        Cli::Pressure { value, from, to } => {
            if from == to {
                println!("{:.4} {} = {:.4} {}", value, from, value, to);
            } else if let Some(result) = convert_pressure(value, from.clone(), to.clone()) {
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
}