use std::thread;
use std::time::Duration;

// Gravitational constant (in m^3 kg^-1 s^-2)
const G: f64 = 6.67430e-11;

// Function to calculate mass using Newton's law of gravitation
fn calculate_mass(force: f64, distance: f64) -> f64 {
    force * distance.powi(2) / G
}

fn main() {
    // Define the gravitational forces and distances for the Sun, Earth, and Moon
    let force_sun = 3.54e22; // Example force in Newtons
    let distance_sun = 1.496e11; // Distance from Earth to Sun in meters

    let force_earth = 9.8; // Example force in Newtons (gravitational force on Earth's surface)
    let distance_earth = 6.371e6; // Radius of Earth in meters

    let force_moon = 1.62; // Example force in Newtons (gravitational force on Moon's surface)
    let distance_moon = 1.737e6; // Radius of Moon in meters

    // Spawn a thread to calculate the mass of the Sun
    let sun_thread = thread::spawn(move || {
        let mass_sun = calculate_mass(force_sun, distance_sun);
        println!("Mass of the Sun: {:.2e} kg", mass_sun);
    });

    // Spawn a thread to calculate the mass of the Earth
    let earth_thread = thread::spawn(move || {
        let mass_earth = calculate_mass(force_earth, distance_earth);
        println!("Mass of the Earth: {:.2e} kg", mass_earth);
    });

    // Spawn a thread to calculate the mass of the Moon
    let moon_thread = thread::spawn(move || {
        let mass_moon = calculate_mass(force_moon, distance_moon);
        println!("Mass of the Moon: {:.2e} kg", mass_moon);
    });

    // Wait for all threads to finish
    sun_thread.join().unwrap();
    earth_thread.join().unwrap();
    moon_thread.join().unwrap();
}