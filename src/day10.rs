use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};



fn main() -> io::Result<()> {
    let path = "src/inputs/day10.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut heads: Vec<(usize,usize)> = Vec::new();
    let mut map: Vec<Vec<i32>> = Vec::new();
    // let mut free_space_map:HashMap::<u32,Vec<usize>> = HashMap::new();
    // let mut used_space_map:HashMap::<u32,Vec<usize>> = HashMap::new();

    for (row, line) in reader.lines().into_iter().enumerate() {
        let line = line?;
        let mut numbers:Vec<i32> = Vec::new();
        for (col, ch) in line.chars().enumerate() {
            match ch.to_digit(10) {
                Some(digit) => {
                    numbers.push(digit as i32);
                    if digit == 0{
                        heads.push((row,col));
                    }
                }
                None => println!("Cant convert {} to digit", ch),
            }
        }
        map.push(numbers);
    }
    let mut score = 0;
    let mut score2 = 0;
    for head in heads{
        println!("head : {:?}",head);
        let mut seen: HashSet<(usize,usize)> = HashSet::new();
        let s = find_trail(head, head, &map, &mut seen);
        score += s.0;
        score2 += s.1;
        println!("score {:?}",s);
        // break;
    }

    println!("part 1: {}",score);
    println!("part 2: {}",score2);

    Ok(())
}


// Points are (row,col)
fn find_trail(prev:(usize,usize),current:(usize,usize),map:&Vec<Vec<i32>>,seen: &mut HashSet<(usize,usize)>) -> (i32,i32){

    let mut sum =0;
    let mut sum2 = 0;
    let val = map[current.0][current.1];
    if val == 9 && !seen.contains(&current){

        seen.insert(current);
        return (1,1);
    }
    if val == 9{
        return (0,1);
    }
    // println!("{:?}",seen);
    if current.0 > 0{
        let p = (current.0-1,current.1);
        let val2 = map[p.0][p.1]; 

        if val2 - val == 1 && p != prev {
            let s = find_trail(current, p, map,seen);
            sum += s.0;
            sum2 += s.1;
        }
    }

    if current.0 < map.len() - 1 {
        let p: (usize, usize) = (current.0+1,current.1);
        let val2 = map[p.0][p.1];

        if val2 - val == 1 && p != prev{
            let s = find_trail(current, p, map,seen);
            sum += s.0;
            sum2 += s.1;
        }
        
    }

    if current.1 > 0{
        let p = (current.0,current.1-1);
        let val2 = map[p.0][p.1]; 
        
        if val2 - val == 1 && p != prev{
            let s = find_trail(current, p, map,seen);
            sum += s.0;
            sum2 += s.1;
        }
    }

    if current.1 < map[0].len() - 1{
        let p = (current.0,current.1+1);
        let val2 = map[p.0][p.1]; 
        
        if val2 - val == 1 && p != prev{
            let s = find_trail(current, p, map,seen);
            sum += s.0;
            sum2 += s.1;
        }
        
    }

    (sum,sum2)

}
