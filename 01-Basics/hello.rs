// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

fn hello() {
    // Print text to the console.
    println!("Hello World!");
    println!("On va apprendre Rust");
}

fn play_with_integers() {
    let n1 = 2_000_000_000;
    let n2 = 255u8;
    let n3: i128 = 16_000_000_000_000_000_000_000_000_000_000_000_005;
    let n4: f64 = n3 as f64; // casting
    let n5: f64 = 0.0 / 0.0;
    let n6: f64 = 1E308 * 4.0;
    println!("n1 = {n1}");
    println!("n2 = {}", n2);
    println!("Numbers: {0}, {1}, {2}, {3}, {4}, {5}", 
            n1, n2, n3, n4, n5, n6);
}

fn analyze_extract(extract: Option<&str>) {
    match extract {
        Some("To") => println!("Special extract To"),
        Some(text) => {
            println!("----------------");
            println!("Extract ok: {text}")
        },
        None => println!("No extract")
    }
}

fn play_with_text() {
    let c1 = '\u{1F99C}';
    let text = "Emoji: 🦜";
    let c2: char = '東';
    let city: &str = "東京";
    let city2: &'static str = "Київ";
    let city3 = "Toulouse";
    println!("Emoji: {c1}");
    println!("{text}");
    println!("City {city} contains kanji {c2}"); 
    println!("City 2 {city2}");
    println!("{}: size = {} bytes, {} graphemes", 
            city3, city3.len(), city3.chars().count());
    println!("{}: size = {} bytes, {} graphemes", 
            city, city.len(), city.chars().count());
    let letter = 'T';
    let prefix = "tou";
    println!("City {} starts with {}: {}", city3, letter, city.starts_with(letter));
    println!("City {} starts with {}: {}", city3, prefix, 
            city3.to_lowercase().starts_with(prefix));
    let field = " \n\r Toulouse \t\r\n  ";
    println!("Input trimmed: #{}#", field.trim());
    // &city3[15..] :  panic => out of bounds
    let extract1: &str = &city3[..3];
    println!("Extracts {}: {}, {}, {}", 
            city3, extract1, &city3[3..6], &city3[6..]);
    let extract2 = city3.get(..3);
    let extract3: Option<&str> = city3.get(15..);
    println!("Extracts {}: {:?}, {:?}", city3, extract2, extract3);

    analyze_extract(extract2);
    analyze_extract(extract3);
    analyze_extract(city3.get(..2));
    analyze_extract(city.get(..3));
    analyze_extract(city.get(..2));
    println!("2nd grapheme of {}: {:?}", city, city.chars().nth(1))
}

// This is the main function.
fn main() {
  hello();
  play_with_integers(); 
  play_with_text(); 
    
}