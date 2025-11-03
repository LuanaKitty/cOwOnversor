use std::io;

pub fn conversion() {
    println!("\nWhich unit to convert from?\n(1) meters  (2) kilometers  (3)  centimeters\n(4) feet  (5) miles  (6) inches");
    let mut unit1 = String::new();
    let _ = io::stdin().read_line(&mut unit1);
    let unit1: u8 = unit1.trim().parse().unwrap_or(0);

    println!("\nInput your unit:");
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let mut input: f64 = input.trim().parse().expect("Invalid input!");

    println!("\nWhich unit to convert to?\n(1) meters  (2) kilometers  (3)  centimeters\n(4) feet  (5) miles  (6) inches");
    let mut unit2 = String::new();
    let _ = io::stdin().read_line(&mut unit2);
    let unit2: u8 = unit2.trim().parse().unwrap_or(0);

    // convert to meters
    match unit1 {
        1 => input *= 1.0,
        2 => input *= 1000.0,
        3 => input /= 100.0,
        4 => input *= 0.3048,
        5 => input *= 1609.34,
        6 => input *= 0.0254,
        _ => {
            println!("Invalid input!");
            return;
        }
    }

    // convert from meter to desired unit
    match unit2 {
        1 => input *= 1.0,
        2 => input /= 1000.0,
        3 => input *= 100.0,
        4 => input *= 3.280839895,
        5 => input *= 0.00062137,
        6 => input *= 39.3700787402,
        _ => {
            println!("Invalid input!");
            return;
        }
    }

    // print the desired unit abbreviation
    let mut unit0 = "m";

    match unit2 {
        1 => unit0 = "m",
        2 => unit0 = "km",
        3 => unit0 = "cm",
        4 => unit0 = "ft",
        5 => unit0 = "mi",
        6 => unit0 = "in",
        _ => {
            println!("Invalid Input!");
            return;
        }
    }
    println!("the conversion is {input}{unit0}");
}
