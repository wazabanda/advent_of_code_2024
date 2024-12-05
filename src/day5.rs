use std::fs::File;
use std::io::{self,BufRead};
//use std::path::Path;
use std::collections::HashMap;


fn main() -> io::Result<()> {
    
    let path = "src/inputs/day5.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    //let lines: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();
    let mut comma_set = Vec::new();
    
    let mut rule_map: HashMap<i32,Vec<i32>> =HashMap::new();
    let mut rule_map_print_after: HashMap<i32,Vec<i32>> =HashMap::new();

    for line in reader.lines(){
        let line = line?;
    
        if line.contains('|'){
            let parts: Vec<i32> = line
                .split('|')
                .map(|s| s.parse::<i32>()
                .unwrap_or(0)).collect();
            
            rule_map
                .entry(parts[0])
                .or_insert_with(Vec::new)
                .push(parts[1]);

            rule_map_print_after
                .entry(parts[1])
                .or_insert_with(Vec::new)
                .push(parts[0]);
        }
        else if line.contains(','){
            let parts: Vec<i32> = line
                .split(',')
                .map(|s| s.parse::<i32>()
                .unwrap_or(0)).collect();
            
            comma_set.push(parts);
        }
    
    }
    
    //let mut valid:Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut sum2 = 0;
    for update in &comma_set{
        //println!("{:?}",update);
        let is_valid = check_order(&update,&rule_map,0 as usize);


        if is_valid.0{
            //println!("is valid: {:?}",update);
            sum += update[update.len()/2]
        }else{
            //println!("is valid reorder: {:?}",update);
            let mut mut_up = update.clone();
            sum2 += reorder(&mut mut_up, &rule_map, is_valid.1)
        }
    }

    println!("part 1: {}",sum);
    println!("part 2: {}",sum2);

    Ok(())
}


fn reorder(update: &mut Vec<i32>, rule_map: &HashMap<i32,Vec<i32>>, start_index:usize) -> i32{

    //let mut slice = &update[start_index..update.len()];
    let mut is_valid = check_order(update,&rule_map,start_index);
    while !is_valid.0{
        
        if is_valid.2.1 != -1{
            update[is_valid.2.0] = update[is_valid.1];
            update[is_valid.1] = is_valid.2.1;
        }else{
            let len = update.len() - 1;
            let t = update[len]; 
            update[len] = update[is_valid.1];
            update[is_valid.1] = t;
        
        }
        is_valid = check_order(update,&rule_map,start_index);
        //println!("{:?}",update);
    } 
    update[update.len()/2]
}


fn check_order(update: &Vec<i32>, rule_map: &HashMap<i32,Vec<i32>>,start_index: usize) -> (bool,usize,(usize,i32)) {

    for (i,order) in update.iter().enumerate(){
        if i < start_index{
            continue;
        } 
        let rule_set: Option<&Vec<i32>> = rule_map.get(order);

        if let Some(rule_set) = rule_set {
            // Use the borrowed rule_set here
            let slice = &update[i+1..update.len()];

            let res = slice
                .iter()
                .all(|&item| rule_set.contains(&item));

            if !res{
                let first_none_matching_element = slice
                    .iter()
                    .enumerate()
                    .find(|&(_, &item)| !rule_set.contains(&item))
                    .map(|(index,&item)| (index+i+1,item));
                
                //println!("{:?}",first_none_matching_element);
                //println!("invalid because: {}",order);
                match first_none_matching_element{
                    Some(tuple) => return (false,i,tuple),
                    None => return (false,i,(0,-1)),
                }
                
            }
        }else{
            if i != update.len() - 1 {

                //println!("No rule found for: {}",order);
                return (false, i,(0,-1));
            }
        }
    }

    (true, 0,(0,-1))
}

