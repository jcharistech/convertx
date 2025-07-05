#### Convertx
### Unit Converter CLI

A simple Rust command-line tool for converting between a wide range of units in categories such as **length, temperature, mass, data rate, area, volume, speed, pressure, bytes, time and more**.

[![Stable](https://img.shields.io/badge/docs-stable-blue.svg)](https://jcharistech.github.io/convertx/)
[![Dev](https://img.shields.io/badge/docs-dev-blue.svg)](https://jcharistech.github.io/convertx/)
[![](https://img.shields.io/crates/v/convertx.svg)](https://crates.io/crates/convertx)


## Installation
To install `convertx` you can do so via cargo

**1. Install with Cargo:**
```bash
cargo add convertx
```
Or simply put the following in your Cargo.toml.

```
[dependencies]
convertx = "0.1.0"
```

**2. Clone and build:**

Alternatively you can build it from the github repo and optionally install it globally
```sh
git clone https://github.com/jcharistech/convertx.git
cd convertx
cargo build --release
```
The binary will be located in `target/release/convertx`.

**3. (Optional) Install globally:**
```sh
cargo install --path .
```

## Features

- **Length:** meters, kilometers, feet, inches  
- **Temperature:** Celsius, Fahrenheit, Kelvin  
- **Mass/Weight:** kilograms, pounds, ounces  
- **Data Rate:** bits per second (bps), megabits per second (mbps)  
- **Bytes:** supports human-readable and MB conversion  
- **Time:** seconds, human-readable duration  
- **Area:** square meters, square feet, acres, hectares  
- **Volume:** liters, milliliters, cubic meters, cubic inches, gallons  
- **Speed:** meters per second, kilometers per hour, miles per hour, knots  
- **Pressure:** pascal, bar, atm, psi  
- **Easy to extend:** add your own units and categories with minimal code changes  
- **Helpful CLI:** shows usage and supported units on `--help`



## Usage

```sh
convertx  [OPTIONS]
```

Get help for any subcommand:
```sh
convertx  --help
```

## Supported Subcommands

| Subcommand       | Description                        | Example                                                        |
|------------------|------------------------------------|----------------------------------------------------------------|
| bytes            | Convert byte values                | `convertx bytes 1048576 --megabytes`                          |
| time             | Convert seconds to human time      | `convertx time 4000 --human-readable`                         |
| length           | Convert length units               | `convertx length 2 --from meters --to feet`                   |
| temperature      | Convert temperature units          | `convertx temperature 100 --from c --to f`                    |
| mass             | Convert mass/weight units          | `convertx mass 2.5 --from kg --to lb`                         |
| datarate         | Convert data rate units            | `convertx datarate 10 --from mbps --to bps`                   |
| area             | Convert area units                 | `convertx area 1000 --from sqm --to acres`                    |
| volume           | Convert volume units               | `convertx volume 2 --from gallons --to liters`                |
| speed            | Convert speed units                | `convertx speed 60 --from mph --to kph`                       |
| pressure         | Convert pressure units             | `convertx pressure 1 --from atm --to psi`                     |

## Examples

**Bytes:**
```sh
convertx bytes 1048576 --megabytes
convertx bytes 1048576 --human-readable
```

**Time:**
```sh
convertx time 4000 --human-readable
```

**Length:**
```sh
convertx length 10 --from kilometers --to meters
convertx length 2 --from feet --to inches
```

**Temperature:**
```sh
convertx temperature 32 --from f --to c
convertx temperature 300 --from k --to f
```

**Mass:**
```sh
convertx mass 100 --from lb --to kg
convertx mass 500 --from oz --to lb
```

**Data Rate:**
```sh
convertx datarate 12345678 --from bps --to mbps
convertx datarate 15 --from mbps --to bps
```

**Area:**
```sh
convertx area 4046.86 --from sqm --to acres
convertx area 2 --from hectares --to sqft
```

**Volume:**
```sh
convertx volume 3.5 --from gallons --to liters
convertx volume 2 --from cubic_meters --to gallons
```

**Speed:**
```sh
convertx speed 60 --from mph --to kph
convertx speed 25 --from mps --to knots
```

**Pressure:**
```sh
convertx pressure 2 --from bar --to psi
convertx pressure 101325 --from pa --to atm
```

## Extending & Contributing

- Add new units by adding an enum variant, conversion function, and CLI case!
- PRs and improvements very welcome.


---


**Jesus Saves @JCharisTech**

**Enjoy lightning-fast, accurate unit conversions from your terminal!**

---


