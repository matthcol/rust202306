use core::fmt;
use std::ops::AddAssign;
// use std::fmt; 


#[derive(Debug)]
struct City{
    name: String,
    population: u32,
    country: String
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
    fn new(name: String, population: u32, country: String) -> City {
        City {
            name: name,
            population: population,
            country: country
        }
    }

    fn is_small(&self) -> bool {
        self.population < 100_000u32
    }
}

impl City {
    fn is_medium(&self) -> bool {
        (self.population >= 100_000u32) && (self.population < 1_000_000u32)
    }

    fn incr_population(&mut self, delta: u32) {
        self.population += delta;
    }
}

fn play_with_one_city() {
    let mut city = City {
        name: String::from("Toulouse"),
        population: 471_941,
        country: String::from("France")
    };
    println!("{city:?}");
    println!("{city}");
    city.incr_population(3u32);
    println!("{city}");
    city += 3u32;
    println!("{city}");
    println!()
}

fn analyze_cities(cities: &Vec<City>) {
    for city in cities {
        match city.country.as_str() {
            "Japan" => println!("A city from Japan"),
            "France"=> println!("A french city"),
            country => println!("City from another country: {country}"),

        }
        match city {
            City{name, population: population @ 0..=99_999u32, country: _} => 
                    println!("Small city[{population}]: {name}"),
            City{name, population: 100_000u32..=999_999u32, ..} => 
                    println!("Medium city: {name}"),
            City{country, name, ..} if country == "Japan" => 
                    println!("Megalopole from Japan: {}", name),
            _ =>  println!("Megalopole not in Japan: {}", city.name)
        }
        println!("Small: {}", city.is_small());
        println!("Medium: {}", city.is_medium());
        println!()
    }
}

fn analyse_small_cities(cities: &Vec<City>){
    for city in cities {
        if let City { 
            name, 
            population: population @ 0u32..=99_999u32, 
            .. } = city 
        {
            println!("Small city: {name} (population = {population})")
        }
    }
}

fn play_with_cities() {
    let cities = vec![
        City {
            name: String::from("Toulouse"),
            population: 471_941,
            country: String::from("France")
        },
        City {
            name: String::from("東京"),
            population: 13_988_129,
            country: String::from("Japan")
        },
        City {
            name: String::from("Pau"),
            population: 75_665,
            country: String::from("France")
        },
        City::new(
            String::from("京都市"), 
            1_460_000u32, 
            String::from("Japan")
        )
    ];
    println!("{cities:?}");
    analyze_cities(&cities);
    analyse_small_cities(&cities);

    // population totale
    let total_population: u32 = cities.iter()
        .map(|c| c.population)
        .sum();

    // population totale d'un pays donné
    let country = "Japan";
    let country_population: u32 = cities.iter()
        .filter(|c| c.country == country)
        .map(|c| c.population)
        .sum();

    println!("Population: total = {1} ; {0} = {2}", 
            country, total_population, country_population);

    let city_max_opt = cities.iter()
        .max_by(|c1, c2| c1.population.cmp(&c2.population));
    // println!("City with population max = {city_max_opt:?}");
    if let Some(city_max) = city_max_opt {
        println!("City with population max = {city_max}");
    }

    // y a t il une ville "small"

    // est ce que toutes les villes du japon sont grandes
}

fn main() {
    play_with_one_city();
    play_with_cities();
}
