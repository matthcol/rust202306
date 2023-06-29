use std::collections::*;

#[derive(Debug)]
pub struct City{
    pub name: String,
    pub population: u32,
    pub country: String
}

fn play_with_collections() {
    // BTreeSet<String>
    let mut city_set = BTreeSet::from([
        String::from("Toulouse"),
        String::from("Bordeaux"),
        String::from("Pau")
    ]);
    println!("City set: {city_set:?}");
    city_set.insert(String::from("Paris"));
    city_set.insert(String::from("Marseille"));
    println!("City set: {city_set:?}");

    // LinkedList<&String>
    let mut city_list = LinkedList::from_iter(city_set.iter());
    println!("City list: {city_list:?}");
    let city = String::from("Lyon");
    city_list.push_back(&city);
    println!("City list: {city_list:?}");

    let mut city_list2 = LinkedList::from_iter(city_set); // call city_set.into_iter()
    city_list2.push_front("Lyon".to_string());
    println!("City list (2): {city_list2:?}");
    // println!("City set: {city_set:?}"); // value borrowed here after move (by into_iter)
    
    // BTreeSet<String>
    let mut city_set2 = BTreeSet::from_iter(city_list2);
    println!("City set (2): {city_set2:?}");
    city_set2.append(&mut BTreeSet::from(["Bayonne".to_string(), "Blagnac".to_string()]));
    println!("City set (2): {city_set2:?}");
}

fn play_with_map(){
    // BTreeMap<&str, i32>
    let mut city_pop_map = BTreeMap::from([
        ("Toulouse", 470000),
        ("Pau", 77000)
    ]);
    println!("City populations: {city_pop_map:?}");
    assert!(city_pop_map.contains_key("Toulouse"));
    println!("Population Toulouse: {:?}", city_pop_map.get("Toulouse"));
    
    for city in ["Pau", "Lyon"] {
        if let Some(pop) = city_pop_map.get(city) {
            println!("Population de {city}: {pop}")
        } else {
            println!("Pas de population pour {city}")
        }
    }
    city_pop_map.insert("Lyon", 500000);
    println!("City populations: {city_pop_map:?}");
}

fn play_with_map_struct(){
    // BTreeMap<&str, City>
    let index_cities = BTreeMap::from([
        ("Toulouse", City{name:"Toulouse".to_string(),population:470000,country:"France".to_string()}),
        ("Pau", City{name:"Pau".to_string(),population:77000,country:"France".to_string()}),
    ]);
    println!("Index: {index_cities:?}");
    if let Some(city) = index_cities.get("Toulouse") {
        println!("City found: {city:?}")
    }
    println!("Index: {index_cities:?}");
}

fn main() {
    play_with_collections();
    play_with_map();
    play_with_map_struct();
}
