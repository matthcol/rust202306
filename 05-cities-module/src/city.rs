pub mod output;

use core::fmt;
use std::ops::AddAssign;
// use std::fmt; 


#[derive(Debug)]
pub struct City{
    pub name: String,
    pub population: u32,
    pub country: String
}
// other ideas: lat, lon, list celebrities

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (pop={}, country={})", self.name, self.population, self.country)
    }
}

impl AddAssign<u32> for City {
    fn add_assign(&mut self, rhs: u32) {
        self.population += rhs;
    }
}

impl City {
    pub fn new(name: String, population: u32, country: String) -> City {
        City {
            name: name,
            population: population,
            country: country
        }
    }

    pub fn is_small(&self) -> bool {
        self.population < 100_000u32
    }
}

impl City {
    pub fn is_medium(&self) -> bool {
        (self.population >= 100_000u32) && (self.population < 1_000_000u32)
    }

    pub fn incr_population(&mut self, delta: u32) {
        self.population += delta;
    }
}

