use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut player: Vec<char> = Vec::new();
    let mut computer: Vec<char> = Vec::new();
    for line in reader.lines() {
        let current_line = line.unwrap().to_string();
        let first_char = current_line.chars().next();
        let third_char = current_line.chars().nth(2);
        computer.push(first_char.unwrap());
        player.push(third_char.unwrap());
    }
    println!("computer : {:?}",computer);
    println!("player : {:?}",player);
}
