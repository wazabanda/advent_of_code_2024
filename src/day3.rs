use std::fs::File;
use std::io::{self,BufRead};
use regex::Regex;
//use std::path::Path;

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day3.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines(){
        let line = line?;
        
        for do_sub in line.split_terminator("do()"){
            let parts: Vec<&str> = do_sub.split("don't()").collect();
            if let Some(first_part) = parts.get(0) {
                println!("{}",first_part);
                let res = extract_mul_expressions(first_part);
                
                for (num1 , num2) in res{
                    sum += num1 * num2;
                }
            
            }


        }
    }

    println!("{}", sum);

    Ok(())
}


fn extract_mul_expressions(input: &str) -> Vec<(i32,i32)> {

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
        let num1 = cap[1].parse::<i32>().unwrap();
        let num2 = cap[2].parse::<i32>().unwrap();
        (num1,num2)
    })
    .collect()
}

