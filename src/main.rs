use std::fs::File;
use std::collections::HashMap;
use std::io::{self,BufRead};



#[derive(Clone)]
struct Point{
    row: usize,
    col: usize,
    value: char,
}


#[derive(Clone)]
struct Line{
    p1:Point,
    p2:Point,
    slope:f64,
    on_same_col:bool,
    on_line:Vec<Point>
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value) // Custom format for printing
    }
}


fn main() -> io::Result<()> {
    
    let path = "src/inputs/day8_test.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut map : Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    let mut mappings:HashMap<char,Vec<Point>> = HashMap::new();
    for (row, line) in reader.lines().into_iter().enumerate() {
        let line = line?;
        let mut sections: Vec<char> = Vec::new();

        for (col,ch) in line.chars().enumerate(){
            let point = Point{row,col,value:ch};
            sections.push(ch);
            mappings.entry(ch).or_insert_with(Vec::new).push(point);
        }

        map.push(sections)
    
    }

    println!("{:?}",mappings);

    for (key,value) in mappings.into_iter()
    {
        if key == '.'{continue};

        for (index,point) in value.iter().enumerate(){
            //println!("{} {:?}",key,point);
            for i in index+1..value.len(){
                let mut slope = 0.0;
                let mut on_same_col = false;
                match get_slope(&point,&value[i]){
                    Some(s) => slope = s,
                    None => on_same_col = true,
                }
                let line = Line{p1:point.clone(),p2:value[i].clone(),slope,on_same_col,on_line:Vec::new()};
                println!("point {} {}",point.col,point.row);
                println!("{:?}",generate_points(&point,&value[i],value.len()));

            }
        }
    }
    

    Ok(())
}


fn get_slope(p1:&Point,p2:&Point) -> Option<f64>{
    
    if p1.col == p2.col{
       return None;
    }
    Some((p2.row - p1.row) as f64 / (p2.col - p1.col) as f64)

}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn generate_points(p1:&Point, p2: &Point, steps: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    let x1 =p1.col;
    let y1 =p1.row;
    let x2 = p2.col;
    let y2 = p2.row;

    let dy = y2 - y1;
    let dx = x2 - x1;

    let divisor = gcd(dy.try_into().unwrap(), dx) as usize;
    let step_y = dy / divisor;
    let step_x = dx / divisor;

    let mut current_x = x1;
    let mut current_y = y1;

    points.push((x1, y1));

    for _ in 0..steps {
        current_x += step_x;
        current_y += step_y;
        points.push((current_x, current_y));
    }

    current_x = x1;
    current_y = y1;
    for _ in 0..steps {
        current_x -= step_x;
        current_y -= step_y;
        points.push((current_x, current_y));
    }

    points.sort();
    points.dedup();

    points
}
