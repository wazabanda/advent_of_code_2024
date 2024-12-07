use std::fs::File;
use std::io::{self,BufRead};

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day7.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    for (row_, line) in reader.lines().into_iter().enumerate() {
        let line = line?;
        let sections: Vec<&str> = line.split(":").collect();
        let numbers: Vec<i64> = sections[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        let mut res = 0; 
        
        match sections[0].parse::<i64>(){
            Ok(r) => res = r,
            Err(_) => {
                println!("faild to parse: {}",sections[0]);
                break;
            }
        }
        
        let left = numbers[0];
        let right = numbers[1];

        let slice = &numbers[2..numbers.len()];

        let ans = run_operation(left,right,slice,res);

        
        if ans.0 == res{
            sum += ans.0;
            //println!("answers are: {} {} {} right one is {}",ans.0,ans.1,ans.2,res);
        }if ans.1 == res{
            sum += ans.1;
            //println!("answers are: {} {} {} right one is {}",ans.0,ans.1,ans.2,res);
        }else if ans.2 == res{
            sum += ans.2;
            //println!("answers are: {} {} {} right one is {}",ans.0,ans.1,ans.2,res);
        }
        else {
            //println!("faild to clear case for: {}", line);
        }
    }

    println!("part 1: {}", sum);
    
    Ok(())
}

fn run_operation(left:i64,right:i64,others:&[i64],check: i64) -> (i64,i64,i64){
    
    //println!("cheking {}  for {} and {} with {:?}",check,left,right,others);
    let sum = left + right;
    let prod = left * right;
    
    let conc = format!("{}{}",left.to_string(),right.to_string()).parse::<i64>().unwrap_or_else(|_| {
            panic!("faild to concat")
        }
    );

    if others.len() > 0 && check > 0{
        let next = others[0];
        let others = &others[1..others.len()];
        
        let n_sum = run_operation(sum,next,&others,check);
        let n_prod = run_operation(prod,next,&others,check);
        let n_conc = run_operation(conc,next,&others,check);
        


        if n_sum.0 == check || n_sum.1 == check || n_sum.2 == check{
            return n_sum;
        }
 
        if n_prod.0 == check || n_prod.1 == check || n_prod.2 == check{
            return n_prod;
        }
    
        if n_conc.0 == check || n_conc.1 == check || n_conc.2 == check{
            return n_conc;
        }
    }

    (sum,prod,conc)
}

