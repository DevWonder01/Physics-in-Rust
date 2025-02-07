use std::f64::consts::E;

fn clausius_clapeyron(
    temperature1: f64,
    pressure1: f64,
    temperature2: f64,
    enthalpy_vaporization: f64,
    gas_constant: f64,
) -> f64 {
    let term1 = enthalpy_vaporization / gas_constant;
    let term2 = 1.0 / temperature1 - 1.0 / temperature2;
    pressure1 * E.powf(term1 * term2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clausius_clapeyron() {
        // Example: Water vapor pressure at 90°C given data at 100°C
        let temperature1 = 373.15; // 100°C in Kelvin
        let pressure1 = 101325.0; // 1 atm in Pascals
        let temperature2 = 363.15; // 90°C in Kelvin
        let enthalpy_vaporization = 40650.0; // J/mol
        let gas_constant = 8.31446261815324; // J/(mol*K)

        let expected_pressure = 70117.0; // Approximate value
        let calculated_pressure =
            clausius_clapeyron(temperature1, pressure1, temperature2, enthalpy_vaporization, gas_constant);

        assert!((calculated_pressure - expected_pressure).abs() < 1000.0); // Allow for some error
    }
}

fn main() {
    // Example usage (same as in the test)
    let temperature1 = 373.15; 
    let pressure1 = 101325.0; 
    let temperature2 = 363.15; 
    let enthalpy_vaporization = 40650.0; 
    let gas_constant = 8.31446261815324; 

    let calculated_pressure =
        clausius_clapeyron(temperature1, pressure1, temperature2, enthalpy_vaporization, gas_constant);

    println!("Calculated pressure at 90°C: {:.2} Pa", calculated_pressure);
}