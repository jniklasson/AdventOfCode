use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::env;
use std::collections::{VecDeque, HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut path = Path::new("input");
    if args.len() > 1{
        path = Path::new(&args[1]);
    }
    let file = File::open(&path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines(); 
    let mut num_chars = 0;
    let mut chars: VecDeque<char> = VecDeque::new(); 
    for line in lines {

        for c in line.unwrap().chars() {
            let uniques: HashSet<char> = chars.iter().cloned().collect();
            if chars.len() >= 14 {
                if (chars.contains(&c)) || (uniques.len() < 14) {
                    chars.pop_front();
                }
                else{
                    println!("Num chars: {}", num_chars);
                    break;
                }
            }
            chars.push_back(c);
            num_chars += 1;

        }
    }
}
