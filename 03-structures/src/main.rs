
fn display_vector_f32(v: &Vec<f32>) {
    let n = v.len();
    if n <= 10 {
        println!("{}{:?}", n, v);
    } else {
        println!("{}{:?}...{:?}", n, &v[..5], &v[n-6..])
    }
}

fn play_with_vectors() {
    let v1 = vec![12, 15, 16];
    let cities = vec![
        String::from("Toulouse"),
        String::from("東京"),
        String::from("Pau")
    ];
    let mut temperatures: Vec<f32> = Vec::new(); //empty vector
    let mut temperatures2: Vec<f32> = vec![0.0f32; 1000];

    

    println!("{}{v1:?}", v1.len());
    println!("{}{cities:?}", cities.len());
    display_vector_f32(&temperatures);
    display_vector_f32(&temperatures2);
    
    temperatures.push(15.5f32);
    temperatures.push(27.5f32);
    temperatures.insert(0, -15.3f32);
    display_vector_f32(&temperatures);

    temperatures2.fill(4.5f32);
    display_vector_f32(&temperatures2);

    temperatures.append(&mut temperatures2);
    display_vector_f32(&temperatures);
    display_vector_f32(&temperatures2);
}

fn play_with_string() {
    let mut city = String::from("Toulouse");
    println!("{} with length {} bytes", city, city.len());
    println!("{}", city.to_uppercase());
    city.push_str(", la ville rose");
    city.push('.');
    println!("{} [length={} bytes]", city, city.len());

    let comment = String::from(" Il fait chaud l'été.");
    city = city + &comment;
    println!("{} [length={} bytes, {} chars]", city, city.len(), city.chars().count());
}

fn main() {
    play_with_vectors();
    play_with_string()
}



