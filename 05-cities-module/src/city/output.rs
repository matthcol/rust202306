use super::City;

pub fn print_cities(cities: &Vec<City>) {
    cities.iter().for_each(|c| println!(" - {c}"));
}