use cities::city::*;
use cities::{city, city_s};

fn main() {
    let mut city = City::new("Toulouse".to_string(), 470000, "France".to_string());
    city += 30;
    let city2 = City::new_with_str("Pau", 77000, "France");
    let city3 = city!("Pau", 77000);
    let city4 = city!("Pau", 77000);
    let city5 = city_s!(s "Pau"; 77000; s "France");
    let city6 = city_s!(S "Pau".to_string(); 77000; S "France".to_string());
    for c in [&city, &city2, &city3, &city4, &city5, &city6] {
        println!("{c:?}");
    }    
}
