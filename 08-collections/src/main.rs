use std::collections::*;

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
    
    // BTreeSet<&String>
    let city_set2 = BTreeSet::from_iter(city_list);
    println!("City set (2): {city_set2:?}");

}

fn main() {
    play_with_collections();
}
