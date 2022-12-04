use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;


fn main() {
    let path = Path::new("input");
    let file = File::open(&path).expect("Could not open file");
    let buf_reader = BufReader::new(file);

    let lines = buf_reader.lines();

    let mut dupes: Vec<char> = Vec::new();
    let mut group: Vec<String> = Vec::new();

    for line in lines{
        group.push(line.unwrap());
        if (group.len() % 3) == 0 {
          let tmp =  find_duplicates(&group[0], &group[1]);
          dupes.append(&mut find_duplicates(&tmp.iter().collect::<String>(), &group[2]));
          group.clear();
        }
    }
    let sum_priorites = sum_priorites(&dupes);
    println!("Sum prios: {}", sum_priorites);

}
fn find_duplicates(first_comp: &String, second_comp: &String) -> Vec<char> {
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
    
    let mut sum: u32 = 0;

    for c in dups{
        sum += get_value(*c) as u32;
    }

    return sum;
}

fn get_value(c: char) -> u8 {
    if (c as u8) < ('a' as u8) {
        (c as u8) - ('A' as u8) + 1 + 26
    } else {
        (c as u8) - ('a' as u8) + 1
    }
}
