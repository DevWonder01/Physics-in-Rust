use std::f64::consts::PI;
use assert_approx_eq::assert_approx_eq;
const IDEAL_GAS_CONSTANT: f64 = 8.31446261815324; // J/(mol*K)

fn ideal_gas_law(pressure: f64, volume: f64, temperature: f64) -> f64 {
    pressure * volume / (IDEAL_GAS_CONSTANT * temperature)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ideal_gas_law() {
        // Example: 1 mole of gas at standard temperature and pressure (STP)
        let pressure = 101325.0; // Pa
        let volume = 0.022414; // m^3
        let temperature = 273.15; // K

        let expected_moles = 1.0;
        let calculated_moles = ideal_gas_law(pressure, volume, temperature);

        assert_approx_eq!(expected_moles, calculated_moles, 0.01); 
    }
}

fn main() {
    // Example usage:
    let pressure = 2.0 * 101325.0; // 2 atm
    let volume = 0.01; // m^3
    let temperature = 300.0; // K

    let moles = ideal_gas_law(pressure, volume, temperature);

    println!("Number of moles: {:.3}", moles);
}