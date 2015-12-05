use std::fs::File;
use std::io::Read;


#[allow(unused_must_use)]
fn main() {
    // Safely read in the file.
    let mut file = File::open("resources/parens.txt").unwrap();
    let mut chars = String::new();
    file.read_to_string(&mut chars);

    // For each character, turn it into a number, then add them all together
    let last_floor = chars.chars()
        .map(|char| match char { '(' => 1, ')' => -1, _ => 0 } )
        .fold(0, |acc, mv| acc + mv);

    println!("Functional: {}", last_floor);
}

