
fn loop_for() {
    let b = 10;
    for i in 1..b {
        println!("- {i}");
        if i % 7 == 0 {
            break;
        }
    }
    for i in 0..=10 {
        println!("* {i}");
    }
    let city = "Toulouse";
    for c in city.chars() {
        println!("~ letter: {c}");
    }
    let city = "東京";
    for c in city.chars() {
        println!("~ letter: {c}");
    }
    let cities = "Toulouse\n東京\r\nPau";
    for city in cities.lines(){
        println!("# {city}");
    }
    let tab = [12, 34, 56, 33];
    for v in tab { // implicit call .iter()
        println!("- {v}");
    }
    for v in tab.iter() {
        println!("* {v}");
    }
    for v in tab.into_iter() {
        println!("~ {v}");
    }
    println!("{tab:?}");
    let vect: Vec<i32> = vec![12, 34, 56, 33];
    for v in vect.into_iter() {
        println!("* {v}");
    }
    // println!("{vect:?}"); vect has been moved

}

fn loop_loop() {
    let mut n = 0;
    loop {
        if n ==  700 {
            // println!("One more...");
            break;
        }
        n += 1;
    }
    println!("{n}");
}

fn loop_while() {
    let mut n = 0;
    while n < 700 {
        n += 1;
    }
    println!("{n}");
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    // let mut a = a;
    // let mut b = b;
    while b != 0 {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    // return value a
    a
}

fn div_mod_panic(a: i32, b: i32) -> (i32, i32) {
    if b == 0 {
        panic!("attempt div_mod by zero")
    }
    (a / b, a % b)
}

fn div_mod_error(a: i32, b: i32) -> Result<(i32, i32), String> {
    if b == 0 {
        return Err(String::from("attempt div_mod by zero"))
    }
    Ok((a / b, a % b))
}

fn play_euclide() {
    let a = 12;
    let b = 15;
    println!("Gcd of {} and {} is {}", a, b, gcd(a, b));
    let (q, r) = div_mod_panic(b, a);
    let qr = div_mod_panic(b, a);
    println!("Euclide division {b} by {a}: q = {q} ; r = {r}");
    println!("Euclide division {b} by {a}: q&r = {qr:?}");
    println!("Euclide division {b} by {a}: q = {} ; r = {}",
            qr.0, qr.1);
}

fn match_integers(x: u8) {
    // need to cover all values
    match x {
        1u8 => println!("Choice 1"),
        0u8 | 2u8 | 4u8 | 6u8 => println!("Even choice 0, 2, 4 or 6"),
        7u8..=255u8 => println!("Choice from 7 to 255"),
        3u8 | 5u8 => println!("Odd choice 3 or 5"),
        // _ => {}
    }
}

fn match_tuple(a: i32, b: i32) {
    match div_mod_error(a, b) {
        Ok((0, r)) => println!("Q0 with remainder {r}"),
        Ok((1, r)) => println!("Q1 with remainder {r}"),
        Ok((q, r)) => println!("Q>1 {q} with remainder {r}"),
        // Err(s) => println!("Error while computing div_mod: {s}")
        Err(_) => println!("Error while computing div_mod")
    }
}

fn main() {
    loop_for();
    loop_loop();
    loop_while();
    play_euclide();
    match_integers(1u8);
    match_integers(3u8);
    match_integers(2u8);
    match_integers(255u8);
    match_tuple(3, 5);
    match_tuple(7, 5);
    match_tuple(12, 5);
    match_tuple(3, 0);
}
