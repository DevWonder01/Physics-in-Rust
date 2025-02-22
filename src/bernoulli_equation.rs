#[derive(Debug, Clone, Copy)]
struct FluidPoint {
    pressure: f64, 
    velocity: f64,
    height: f64, 
}

const GRAVITY: f64 = 9.81; // m/s^2
const TOLERANCE: f64 = 1e-3; // Small tolerance for floating-point comparisons

fn bernoulli_equation(fluid_density: f64, point1: FluidPoint, point2: FluidPoint) -> bool {
    let bernoulli_constant1 = 
        point1.pressure / fluid_density + 0.5 * point1.velocity.powi(2) + GRAVITY * point1.height;

    let bernoulli_constant2 = 
        point2.pressure / fluid_density + 0.5 * point2.velocity.powi(2) + GRAVITY * point2.height;

    (bernoulli_constant1 - bernoulli_constant2).abs() < TOLERANCE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bernoulli_equation_valid() {
        // Example: Water flowing in a horizontal pipe (height remains constant)
        let fluid_density = 1000.0; // kg/m^3 (density of water)
        let point1 = FluidPoint { pressure: 100000.0, velocity: 2.0, height: 0.0 };
        let point2 = FluidPoint { pressure: 99800.0, velocity: 2.2, height: 0.0 };

        assert!(bernoulli_equation(fluid_density, point1, point2)); 
    }

    #[test]
    fn test_bernoulli_equation_invalid() {
        // Example: Inconsistent values for Bernoulli's equation
        let fluid_density = 1000.0; 
        let point1 = FluidPoint { pressure: 100000.0, velocity: 2.0, height: 0.0 };
        let point2 = FluidPoint { pressure: 90000.0, velocity: 1.0, height: 5.0 }; 

        assert!(!bernoulli_equation(fluid_density, point1, point2)); 
    }
}

fn main() {
    let fluid_density = 1000.0; 
    let point1 = FluidPoint { pressure: 100000.0, velocity: 2.0, height: 0.0 };
    let point2 = FluidPoint { pressure: 99800.0, velocity: 2.2, height: 0.0 };

    if bernoulli_equation(fluid_density, point1, point2) {
        println!("Bernoulli's equation is satisfied.");
    } else {
        println!("Bernoulli's equation is not satisfied.");
    }
}
