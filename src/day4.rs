
use std::fs::File;
use std::io::{self,BufRead};
use regex::Regex;
//use std::path::Path;

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day4.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    let mut sum2 = 0;
    let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let hor = lines[0].len();
    let ver = lines.len();

    for v in 0..ver{
        let line = &lines[v];
        sum += check_horizontal(&line);
        let vert_slice = &lines[v..usize::min(v+4,lines.len())];
        sum += check_vertical(vert_slice);
        sum += check_diagonal(vert_slice,true,"XMAS");
        sum += check_diagonal(vert_slice,false,"XMAS");

        if v > 0 && v < ver -1{
            let vert_slice = &lines[usize::min(v-1,line.len())..usize::min(v+2,lines.len())];
            //println!("{:?}",vert_slice);
            sum2 += check_diagonal_2(vert_slice);
        }
    }

    println!("part 1: {}", sum);
    println!("part 2: {}", sum2);



    Ok(())
}


fn check_horizontal(line: &String) -> i32{
    
    let mut count = 0;
    for (index,cha) in line.chars().enumerate()
    {
        let end = usize::min(index+4,line.len());
        let substring:String = (&line[index..end]).to_string();
        let reversed:String = substring.chars().rev().collect();
        if substring == "XMAS" || reversed == "XMAS"{
            count += 1;
        }

    }
    count
}

fn check_str(res:&String,check: &str) -> bool
{
    
    if res == check {
        
        return true
    }
    
    if res.chars().rev().collect::<String>() == check{

        return true
    }

    return false;
}

fn check_diagonal_2(lines: &[String]) -> i32{
    let hor = lines[0].len();
    let mut count = 0;
    for (index,c) in lines[1].chars().into_iter().enumerate() {
        let mut res = String::new();
        let mut res2 = String::new();
        if c == 'A' && index > 0
        {
            if let Some(c) = lines[0].chars().nth(index-1){
                res.push(c);
            }
            res.push('A');
            if let Some(c) = lines[2].chars().nth(index+1){
                res.push(c);
            }
            let first = check_str(&res,"MAS"); 

            res.clear();

            
            if let Some(c) = lines[0].chars().nth(index+1){
                res2.push(c);
            }
            res2.push('A');
            if let Some(c) = lines[2].chars().nth(index-1){
                res2.push(c);
            }
            let second = check_str(&res2,"MAS"); 
            
            if first && second{
                count += 1
            }
            
            
        }
    
    }
    count
}

fn check_diagonal(lines: &[String], forward:bool, check: &str) -> i32{
    let hor = lines[0].len();
    let mut count = 0;
    for i in 0..hor {
        let mut res = String::new();
        
        for (index,line) in lines.into_iter().enumerate() {
            let mut ind =  0 ;
            if forward {ind = i + index;}
            else{
                if i < 3{
                    break;
                }else{
                    ind = i - index;
                }
            }
            if ind >= hor {
                break;
            }
            if let Some(c) = line.chars().nth(ind) {
                res.push(c);
            }
        }

        if res == check || res.chars().rev().collect::<String>() == check{
            count += 1;
        }
    }

    count
}

fn check_vertical(lines: &[String]) -> i32 {
    let mut count = 0;
    let hor = lines[0].len();
    for i in 0..hor {
        let mut res = String::new();
        
        for line in lines.iter() {
            if let Some(c) = line.chars().nth(i) {
                res.push(c);
            }
        }

        if res == "XMAS" || res.chars().rev().collect::<String>() == "XMAS"{
            count += 1;
        }
    }
    count
}

