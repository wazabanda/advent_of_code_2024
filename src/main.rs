use std::fs::File;
use std::io::{self,BufRead};
//use std::path::Path;

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day1.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut left_numbers = vec![];
    let mut right_numbers = vec![];

    for line in reader.lines(){
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        left_numbers.push(numbers[0]);
        right_numbers.push(numbers[1]);
    }

    left_numbers.sort();
    right_numbers.sort();
    
    let ans: i32 = left_numbers
        .iter()
        .zip(right_numbers.iter())
        .map(|(a,b)| (a-b).abs())
        .sum();

    println!("{}", ans);

    Ok(())
} 
