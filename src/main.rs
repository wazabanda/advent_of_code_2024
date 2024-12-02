use core::num;
use std::fs::File;
use std::io::{self,BufRead};

//use std::path::Path;

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day2.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut safe = 0;

    let mut unsafe_logs:Vec<Vec<i32>>  = Vec::new();

    for line in reader.lines(){
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        let mut is_safe = true;

        is_safe = process_line(&numbers);

        if is_safe{
            safe += 1;
        }else{
            unsafe_logs.push(numbers);
        }
    }

    for number in unsafe_logs{
        for i in 0..number.len(){
            let mut number_removed = number.clone();
            number_removed.remove(i);

            if process_line(&number_removed){
                safe += 1;
                break;
            }
        }
    }

    println!("how many are safe: {}",safe);


    Ok(())
} 

fn is_sorted_ascending<T: Ord>(vec: &[T]) -> bool{
    vec.windows(2).all(|w| w[0] <= w[1])
}

fn is_sorted_descending<T: Ord>(vec: &[T]) -> bool{
    vec.windows(2).all(|w| w[0] >= w[1])
}

fn process_line(numbers:&Vec<i32>) -> bool
{
        // let mut prev = 0;
        let mut is_safe = true;

        let mut incr = is_sorted_ascending(&numbers);
        
        let mut unsafe_count = 0;

        numbers.windows(2).all(|w|{
            let prev = w[0];
            let num = w[1];
            let dis = (prev-num).abs();

            if (incr && prev > num) || (incr == false && prev < num) || prev == num || dis < 1 || dis > 3{
                return  false;
            }
            true
        })
        
}