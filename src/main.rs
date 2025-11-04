mod length;
mod speed;
use std::io;

fn main() {
    println!("Pick a type of conversion!\n(1) Area   (2) Speed   (3) Length   (4) Time   (5) Temperature");
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);
    let input: u8 = input.trim().parse().unwrap_or(0);
    match input {
        0 => Some(println!("Invalid input!")),
        1 => todo!(),                   // todo
        2 => Some(speed::conversion()), 
        3 => Some(length::conversion()),
        4 => todo!(), // todo
        5 => todo!(), // todo
        _ => Some(println!("Invalid input!")),
    }
    .expect("Invalid input")
}
