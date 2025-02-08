use nalgebra::{Vector3, Point3};

const GRAVITATIONAL_CONSTANT: f64 = 6.67430e-11; // m^3 kg^-1 s^-2

/// Calculates the gravitational force between two objects.
///
/// # Arguments
///
/// * `mass1`: The mass of the first object (kg).
/// * `mass2`: The mass of the second object (kg).
/// * `position1`: The position of the first object (meters).
/// * `position2`: The position of the second object (meters).
///
/// # Returns
///
/// A `Vector3` representing the gravitational force exerted on the first object by the second object.
fn gravitational_force(
    mass1: f64,
    mass2: f64,
    position1: &Point3<f64>,
    position2: &Point3<f64>,
) -> Vector3<f64> {
    let distance_vector = position2 - position1;
    let distance_squared = distance_vector.norm_squared();
    let distance = distance_vector.norm();

    // Avoid division by zero
    if distance_squared == 0.0 {
        return Vector3::zeros();
    }

    let force_magnitude = GRAVITATIONAL_CONSTANT * mass1 * mass2 / distance_squared;
    let force_direction = distance_vector / distance;

    force_magnitude * force_direction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravitational_force() {
        // Example: Two objects with masses 1 kg and 2 kg, separated by 1 meter
        let mass1 = 1.0;
        let mass2 = 2.0;
        let position1 = Point3::new(0.0, 0.0, 0.0);
        let position2 = Point3::new(1.0, 0.0, 0.0);

        let expected_force_magnitude = GRAVITATIONAL_CONSTANT * mass1 * mass2;
        let expected_force_direction = Vector3::new(1.0, 0.0, 0.0);

        let calculated_force = gravitational_force(mass1, mass2, &position1, &position2);

        assert_eq!(calculated_force.magnitude(), expected_force_magnitude);
        assert_eq!(calculated_force.normalize(), expected_force_direction);
    }
}