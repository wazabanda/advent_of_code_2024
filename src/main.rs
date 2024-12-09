use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "src/inputs/day9_test.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut rule: Vec<i32> = Vec::new();
    // let mut free_space_map:HashMap::<u32,Vec<usize>> = HashMap::new();
    // let mut used_space_map:HashMap::<u32,Vec<usize>> = HashMap::new();
    let mut free_space_map: Vec<usize> = Vec::new(); //stores the indices of where blocks will end;
    let mut used_space_map: Vec<usize> = Vec::new(); //stores the indices of where blocks will end;
    let mut last_index = 0;
    for (row, line) in reader.lines().into_iter().enumerate() {
        let line = line?;
        let mut index = 0;
        for (col, ch) in line.chars().enumerate() {
            match ch.to_digit(10) {
                Some(digit) => {
                    last_index += digit;
                    if col % 2 == 0 {
                        used_space_map.push((last_index - 1).try_into().unwrap());

                        // used_space_map.entry(digit).or_insert_with(Vec::new).push(rule.len());
                        for _ in 0..digit {
                            rule.push(index);
                        }
                        index += 1;
                    } else {
                        free_space_map.push((last_index - 1).try_into().unwrap());

                        // free_space_map.entry(digit).or_insert_with(Vec::new).push(rule.len());
                        for _ in 0..digit {
                            rule.push(-1);
                        }
                    }
                }
                None => println!("Cant convert {} to digit", ch),
            }
        }
    }

    println!("{:?}", used_space_map);
    println!("{:?}", free_space_map);
    let mut l_check = rule.len() - 1;
    let mut rule2 = rule.clone();
    //println!("used space: {:?}",used_space_map);
    //println!("free space: {:?}",free_space_map);
    for f_check in 0..rule.len() {
        if rule[f_check] != -1 {
            continue;
        }

        let mut v = rule[l_check];

        while v == -1 && l_check > f_check {
            l_check -= 1;
            v = rule[l_check];
        }

        rule[f_check] = v;
        rule[l_check] = -1;
    }

    let mut l_check = rule2.len() - 1;
    let mut current_empty_block: usize = 0;
    let mut changed_block = true;
    let mut f_check = 0;
    let mut outer_scan = 0;
    while f_check < rule2.len() && outer_scan < rule2.len() {
        if rule2[f_check] != -1 {
            f_check += 1;
            changed_block = true;
            continue;
        }

        let mut v = rule2[l_check];

        let len = {
            if current_empty_block < used_space_map.len()
                && current_empty_block < free_space_map.len()
            {
                free_space_map[current_empty_block] - used_space_map[current_empty_block]
            } else {
                0
            }
        };

        println!("Free block len {:?}", len);

        // get tehe first size of the block
        let mut len2 = rule2.len();
        let mut best_start: usize = l_check;
        let mut best_end: usize = l_check;
        let mut l_check_last = l_check;
        let mut scan = 0;
        let mut temp_lcheck: usize = l_check;
        let mut found = false;

        while len2 > len && best_start > f_check && scan < rule2.len() {
            scan += 1;
            while v == -1 && l_check > f_check {
                l_check -= 1;
                v = rule2[l_check];

                if best_end == rule2.len() && best_start == rule2.len() {
                    best_start = l_check;
                    best_end = l_check;
                }
            }

            l_check_last = l_check;

            while l_check_last > 1 && rule2[l_check_last - 1] == v && l_check_last > f_check{
                l_check_last -= 1;
            }
            
            let l = l_check - l_check_last + 1;
            println!(
                "for value {} found end {} at {} scan {}",
                v, rule2[l_check_last], l_check_last, scan
            );
            if l <= len{
                best_end = l_check;
                best_start = l_check_last;
                found = true;
            
            }
            if !found || l <= len {
                l_check = l_check_last - 1;
                
                v = {
                    let mut i = l_check;
                    while rule2[i] == -1 {
                        i -= 1;
                    }
                    l_check = i;
                    rule2[i]
                };
            }
            len2 = l;
        }

        if !found {
            l_check = temp_lcheck;

            break;
        }

        println!(
            "the value {} is between {} and {} with length {} fcheck {}",
            v, best_start, best_end, len2,f_check
        );

        let lenx = best_end + 1 - best_start;
        let temp: Vec<_> = rule2[best_start..best_end + 1].to_vec();

        println!("{:?}",temp);
        rule2[f_check..f_check + lenx].copy_from_slice(&temp);
        for i in best_start..best_end + 1 {
            rule2[i] = -1;
        }
        // println!("{} {}",len,len2);
        println!("{:?}", rule2);
        if changed_block && len2 == len {
            changed_block = false;
            current_empty_block += 1;
            f_check += 1;
            outer_scan = 0;
        } else if changed_block && len2 < len {
            f_check += 1;
            free_space_map[current_empty_block] -= len2;
            outer_scan = 0;
        }
        l_check = temp_lcheck;//l_check_last - 1;
        println!("new fcheck {}", f_check);
        println!("l check {}", l_check);
        outer_scan += 1;
        // break;
    }

    let mut sum1: u64 = 0;

    for (i, v) in rule.into_iter().enumerate() {
        if v != -1 {
            sum1 += i as u64 * v as u64;
        }
    }

    println!("part 1: {}", sum1);

    Ok(())
}
