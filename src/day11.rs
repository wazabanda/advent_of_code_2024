use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};


fn insert_or_increment(map: &mut HashMap<u64, u64>, key: u64) {
        // Use entry to either insert or increment
        let counter = map.entry(key).or_insert(0);
        *counter += 1; // Increment the value by 1
}


fn split_number(num: u64) -> (u64, u64) {
    let num_str = num.to_string();
    let mid = (num_str.len() + 1) / 2; 

    let (first_half, second_half) = num_str.split_at(mid);

    let first = first_half.parse::<u64>().unwrap_or(0);
    let second = second_half.parse::<u64>().unwrap_or(0);

    (first, second)
}

fn main() -> io::Result<()> {
    let path = "src/inputs/day11.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    
    let mut memory_map:HashMap<u64,u64> = HashMap::new();

    let mut numbers:Vec<u64> = Vec::new();
    for (row, line) in reader.lines().into_iter().enumerate() {
        numbers = line?
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        
        println!("{:?}",numbers);
    
    }
    for number in numbers{
        insert_or_increment(&mut memory_map, number);
    }

    // println!("{:?}",&memory_map);

    for i in 0..75{

        let mut temp:HashMap<u64,u64> = HashMap::new();
        for (k,v) in memory_map.clone(){
            if k == 0{
                let counter = temp.entry(1).or_insert(0);
                *counter += v;

            }else if k == 1{
                let counter = temp.entry(2024).or_insert(0);
                *counter += v;
            }else{
                let s_num = k.to_string();
                if s_num.len() % 2 != 0{
                    let counter = temp.entry(k * 2024).or_insert(0);
                    *counter += v;
                }else{
                   let (first,second) = split_number(k);

                    let counter = temp.entry(first).or_insert(0);
                    *counter += v;
                    let counter = temp.entry(second).or_insert(0);
                    *counter += v;
                }
            }

            memory_map = temp.clone();
        }    
    }    

    let mut sum = 0;
    for (k,v) in memory_map{
        sum += v;
    }

    println!("part 1: {}",sum);
    Ok(())
}


