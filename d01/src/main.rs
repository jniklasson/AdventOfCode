use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};

fn main() {

    let mut sum_vec = Vec::new();
    let mut max:u32 = 0;
    let mut sum:u32 = 0;
    let mut top_three: [u32;3] = [0,0,0];
    let mut top_three_sum:u32 = 0;

    let path = Path::new("input");
    let file = File::open(&path)
        .expect("Could not open file");

    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let s = line.unwrap();
        if s.is_empty(){
            sum_vec.push(sum);
            if sum > max{
                max = sum;
            }
            sum = 0;
        }
        else{
            let num: u32 = s.parse().unwrap();
            sum += num;
        }
    }

    for num in sum_vec{
        let index = get_lowest_index(&top_three);
        if num > top_three[index]{
            top_three[index] = num;
        }
    }
    for val in top_three{
        top_three_sum += val;
    }

    println!("Most Calories: {}", max);
    println!("Top three: {} {} {}",
             top_three[0],top_three[1],top_three[2]);
    println!("Sum of top three calories: {}", top_three_sum);

}

fn get_lowest_index(arr: &[u32]) -> usize {
    let mut lowest = u32::MAX;
    let mut index = 0;

    for i in 0..arr.len(){
        if arr[i] < lowest {
            lowest = arr[i];
            index = i;
        }
    }
    index
}
