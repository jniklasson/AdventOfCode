use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input");
    let file = File::open(&path).expect("Could not open file");
    let mut sum_of_prios: u32 = 0; 
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let items = line.unwrap();
        let num_items = items.len();
        let first = &items[0..num_items / 2];
        let second = &items[num_items / 2..];
        let dups: Vec<char> = find_duplicates(first, second);
        sum_of_prios += sum_priorites(&dups);
    }

    println!("Sum of priorities: {}", sum_of_prios);
}

fn find_duplicates(first_comp: &str, second_comp: &str) -> Vec<char> {
    let mut dup: Vec<char> = Vec::new();
    for f in first_comp.chars() {
        for s in second_comp.chars(){
            if f == s{
                if dup.iter().any(|&x| x == f){
                    continue;
                }
                else{
                    dup.push(f);
                }
            }
        }
    }
    return dup;
}
fn sum_priorites(dups: &Vec<char>) -> u32{
    let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum: u32 = 0;

    for (i, c) in letters.chars().enumerate(){
        if dups.iter().any(|&x| x == c){
            sum += i as u32 + 1;
        }
    }
    return sum;
}
