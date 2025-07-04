use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn bytes_megabytes() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["bytes", "1048576", "--megabytes"]);
    cmd.assert().success().stdout(contains("1.00 MB"));
}

#[test]
fn time_human_readable() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["time", "3661", "--human-readable"]);
    cmd.assert().success().stdout(contains("1h 1m 1s"));
}

#[test]
fn length_kilometers_to_meters() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["length", "1", "--from", "kilometers", "--to", "meters"]);
    cmd.assert().success().stdout(contains("1.0000 kilometers = 1000.0000 meters"));
}

#[test]
fn temperature_f_to_c() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["temperature", "32", "--from", "f", "--to", "c"]);
    cmd.assert().success().stdout(contains("32.00°F = 0.00°C"));
}

#[test]
fn mass_kg_to_lb() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["mass", "1", "--from", "kg", "--to", "lb"]);
    cmd.assert().success().stdout(contains("1.0000 kg = 2.2046 lb"));
}

#[test]
fn datarate_mbps_to_bps() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["datarate", "1", "--from", "mbps", "--to", "bps"]);
    cmd.assert().success().stdout(contains("1.0000 mbps = 1000000.0000 bps"));
}

#[test]
fn area_acres_to_sqm() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["area", "1", "--from", "acres", "--to", "sqm"]);
    cmd.assert().success().stdout(contains("1.0000 acres = 4046.8564 sqm"));
}

#[test]
fn volume_gallons_to_liters() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["volume", "1", "--from", "gallons", "--to", "liters"]);
    cmd.assert().success().stdout(contains("1.0000 gallons = 3.7854 liters"));
}

#[test]
fn speed_mph_to_kph() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["speed", "60", "--from", "mph", "--to", "kph"]);
    cmd.assert().success().stdout(contains("60.0000 mph = 96.5606 kph"));
}

#[test]
fn pressure_atm_to_psi() {
    let mut cmd = Command::cargo_bin("convertx").unwrap();
    cmd.args(&["pressure", "1", "--from", "atm", "--to", "psi"]);
    cmd.assert().success().stdout(contains("1.0000 atm = 14.6959 psi"));
}