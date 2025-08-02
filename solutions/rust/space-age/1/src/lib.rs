// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}
impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::ORBITAL_PERIOD
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 0.2408467;
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 0.61519726;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 31_557_600.;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = Earth::ORBITAL_PERIOD * 164.79132;
}
