use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;


fn main() {
    let path = Path::new("input");
    let file = File::open(&path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines(); 
    let mut num_overlap = 0;
    for line in lines{
       let text = line.expect("Invalid line");
       let pairs : Vec<u16> = text
           .split(|c| c == ',' || c == '-')
           .map(|x| x.parse::<u16>().unwrap())
           .collect();
     let s1: HashSet<u16> = (pairs[0]..pairs[1]+1).collect(); 
     let s2: HashSet<u16> = (pairs[2]..pairs[3]+1).collect(); 
    
     if s1.intersection(&s2).count() != 0{
         num_overlap += 1;
     }

    }
   println!("Num overlaps: {}", num_overlap); 
}


