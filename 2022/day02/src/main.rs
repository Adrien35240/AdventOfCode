use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut p1_score: i32 = 0;
    let mut p2_score: i32 = 0;
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut player: Vec<char> = Vec::new();
    let mut computer: Vec<char> = Vec::new();
    for line in reader.lines() {
        let current_line = line.unwrap().to_string();
        let first_char = current_line.chars().next();
        let third_char = current_line.chars().nth(2);

        if let (Some(p1_choice), Some(p2_choice)) = (first_char, third_char) {
            player.push(p1_choice);
            computer.push(p2_choice);
            compare(p1_choice, p2_choice, &mut p1_score, &mut p2_score);
        }
    }
    println!("score computer: {:?}", p1_score);
    println!("score player: {:?}", p2_score);
}

fn compare(p1_choice: char, p2_choice: char, p1_score: &mut i32, p2_score: &mut i32) {
    let p1_possibility: [char; 3] = ['A', 'B', 'C'];
    let p2_possibility: [char; 3] = ['X', 'Y', 'Z'];

    let index_p1 = p1_possibility.iter().position(|&x| x == p1_choice);
    let index_p2 = p2_possibility.iter().position(|&x| x == p2_choice);

    match (index_p1,index_p2) {
        (Some(p1),Some(p2)) if p1 > p2 => {
            *p1_score += p1 as i32 + 6;
            *p2_score += p2 as i32; 
         }
        (Some(p1),Some(p2)) if p2 > p1 => {
            *p2_score += p2 as i32 + 6;
            *p1_score += p1 as i32; 
         }
        (Some(p1),Some(p2)) if p1 == p2 => {
            *p1_score += p1 as i32 + 3;
            *p2_score += p2 as i32 + 3; 
         }
         _ => {}
    }

}
