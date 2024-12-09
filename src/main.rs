use std::fs::File;
use std::collections::HashMap;
use std::io::{self,BufRead};




fn main() -> io::Result<()> {
    
    let path = "src/inputs/day9.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut rule:Vec<i32> = Vec::new();
    let mut free_space_map:HashMap::<u32,Vec<usize>> = HashMap::new();
    let mut used_space_map:HashMap::<u32,Vec<usize>> = HashMap::new();
    
    for (row, line) in reader.lines().into_iter().enumerate() {
        let line = line?;
        let mut index = 0;
        for (col,ch) in line.chars().enumerate(){
            match ch.to_digit(10){
                Some(digit) => {

                    if col % 2 == 0{
                        used_space_map.entry(digit).or_insert_with(Vec::new).push(rule.len());
                        for _ in 0..digit{
                            rule.push(index);
                        }
                        index += 1;
                    }else{
                        free_space_map.entry(digit).or_insert_with(Vec::new).push(rule.len());
                        for _ in 0..digit{
                            rule.push(-1);
                        }
                    }
                },
                None => println!("Cant convert {} to digit",ch)
            }
        }
    }
    let mut l_check = rule.len() - 1;
    let mut part_2_rules = rule.clone();
    for f_check in 0..rule.len(){
        if rule[f_check] != -1 {continue;}
        
        let mut v = rule[l_check];       
        
        while v == -1 && l_check > f_check{
            l_check -= 1;
            v = rule[l_check];
        } 

        rule[f_check] = v;
        rule[l_check] = -1;


    }
    
    let mut sum1:u64 = 0;

    for (i,v) in rule.into_iter().enumerate(){
        if v != -1{
            sum1 += i as u64 * v as u64;
        }
    }   

    println!("part 1: {}",sum1);

    Ok(())
}

