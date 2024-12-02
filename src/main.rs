use std::fs::File;
use std::io::{self,BufRead};
//use std::path::Path;

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day2.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut safe = 0;

    for line in reader.lines(){
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut prev = 0;
        prev = numbers[0];
        let mut is_safe = true;
        let mut incr = true;
        let mut unsafe_count = 0;
        for i in 1..numbers.len(){
            let num = numbers[i];
            
            if i == 1 || unsafe_count == 1{
                incr = prev < num;
            }

            if (incr && prev > num) || (incr == false && prev < num){
                
                is_safe = false;
                unsafe_count += 1;
                //prev = num;
                if unsafe_count > 1{
                    break;
                }
                continue;
            }

            if prev == num{
                is_safe = false;
                unsafe_count += 1;
                //prev = num;     
                if unsafe_count > 1{
                    break;
                }
                continue;
            }
            let dis:i32 = (prev - num).abs();
            if dis >= 1 && dis <= 3{
                prev = num;
            }else{
                is_safe = false;
                unsafe_count += 1;
                //prev = num;
                if unsafe_count > 1{
                    break;
                }
                continue;
            }
        }
        if unsafe_count == 0{
            is_safe = true;
        }
        if is_safe{
            safe += 1;
        }
    }

    println!("how many are safe: {}",safe);


    Ok(())
} 
