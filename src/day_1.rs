// Day 1: http://adventofcode.com/day/1
use std::fs::File;
use std::io::Read;


#[allow(unused_must_use)]
pub fn solutions() {
    let mut file = File::open("resources/parens.txt").unwrap();
    let mut parens = String::new();
    file.read_to_string(&mut parens);

    let last_floor = part_1(&parens);
    let first_basement = part_2(&parens);

    println!("Day 1.");
    println!("  Number of parens: {}", parens.len());
    println!("  Last floor: {}", last_floor);
    println!("  First time in the basement: {}", (first_basement + 1));
}


pub fn part_1(parens: &String) -> isize {
    parens.chars()
        .map(|char| match char { '(' => 1, ')' => -1, _ => 0 } )
        .fold(0, |acc, mv| acc + mv)
}


pub fn part_2(parens: &String) -> usize {
    let mut floor = 0;

    for (i, c) in parens.chars().enumerate() {
        floor += match c { '(' => 1, ')' => -1, _ => 0 };
        if floor < 0 {
            return i;
        }
    }

    return 0;
}
