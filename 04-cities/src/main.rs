use core::fmt;
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

fn play_with_one_city() {
    let city = City {
        name: String::from("Toulouse"),
        population: 471_941,
        country: String::from("France")
    };
    println!("{city:?}");
    println!("{city}");
}

fn analyze_cities(cities: &Vec<City>) {
    for city in cities {
        match city.country.as_str() {
            "Japan" => println!("A city from Japan"),
            "France"=> println!("A french city"),
            country => println!("City from another country: {country}"),

        }
        match city {
            City{name, population: population @ 0u32..=99_999u32, country: _} => 
                    println!("Small city[{population}]: {name}"),
            City{name, population: 100_000u32..=999_999u32, ..} => 
                    println!("Medium city: {name}"),
            City{country, name, ..} if country == "Japan" => 
                    println!("Megalopole from Japan: {}", name),
            _ =>  println!("Megalopole not in Japan: {}", city.name)
        }
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
        }
    ];
    println!("{cities:?}");
    analyze_cities(&cities);
    analyse_small_cities(&cities);
}

fn main() {
    play_with_one_city();
    play_with_cities();
}
