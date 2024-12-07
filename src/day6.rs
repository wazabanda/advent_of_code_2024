use std::fs::File;
use std::io::{self,BufRead};
//use std::path::Path;
use std::collections::HashSet;


#[derive(Debug,Copy,Clone,PartialEq)]
enum PointType{
    Player,
    Obstacle,
    Path,
}

#[derive(Debug)]
enum PlayerDirection{
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Point{
    row: usize,
    col: usize,
    value: String,
    visited: bool,
    point_type: PointType,
    dx: i32,
    dy: i32,
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value) // Custom format for printing
    }
}

fn main() -> io::Result<()> {
    
    let path = "src/inputs/day6.txt";
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut map: Vec<Vec<Point>> = Vec::new();
    let mut visited = 0; 
    let mut player: Point = Point{row:0,col:0,value:"".to_string(),visited:false,point_type:PointType::Path,dx:0,dy:0};
    let mut player_direction: PlayerDirection = PlayerDirection::Right;
    for (row_, line) in reader.lines().into_iter().enumerate() {
        let mut row_points: Vec<Point> = Vec::new();
        for (col_, ch) in line?.chars().enumerate() {
            let mut dx = 0;
            let mut dy = 0;
            let mut p_type:PointType = PointType::Path;

            // Check for player characters and set dx, dy
            match ch {
                '>' => {
                    p_type = PointType::Player;
                    player_direction = PlayerDirection::Right;
                    dx = 1;
                    dy = 0;
                }
                '<' => {
                    p_type = PointType::Player;
                    player_direction = PlayerDirection::Left;
                    dx = -1;
                    dy = 0;
                }
                'v' => {
                    p_type = PointType::Player;
                    player_direction = PlayerDirection::Down;
                    dx = 0;
                    dy = 1;
                }
                '^' => {
                    p_type = PointType::Player;
                    player_direction = PlayerDirection::Up;
                    dx = 0;
                    dy = -1;
                }
                '#' => {
                    p_type = PointType::Obstacle;
                }
                _ => {}
            }
            
            let mut p = Point {
                row: row_,
                col: col_,
                value: ch.to_string(),
                visited: false,
                point_type:p_type,
                dx,
                dy,
            };
            match p.point_type {
                PointType::Player=> {
                        player = p;

                        p = Point {
                            row: row_,
                            col: col_,
                            value: ch.to_string(),
                            visited: true,
                            point_type:PointType::Path,
                            dx,
                            dy,
                        };
                        visited += 1;
                    }
                _ => {
                }
                }
            // Create a new Point and add it to the row
            row_points.push(p);
        }
        //println!("{:?}",row_points);
        map.push(row_points);
    }
    
    let mut has_left = false;
    while !has_left{
        let new_row = usize::min(
            ((player.row as i32) + player.dy).max(0) as usize,  // Ensure row is non-negative and convert back to usize
            map.len() // Ensure new_row doesn't exceed max row index
        );

        let new_col = usize::min(
            ((player.col as i32) + player.dx).max(0) as usize,  // Ensure col is non-negative and convert back to usize
            map[0].len() // Ensure new_col doesn't exceed max column index
        );
        

        if new_row >= map[0].len() || (player.row == 0 && player.dy == -1)
        {
            has_left = true;
            break;
        }
        
        if new_col >= map.len() || (player.col == 0 && player.dx == -1)
        {
            has_left = true;
            break;
        }
        
        

        let point = &mut map[new_row][new_col];

        if point.point_type == PointType::Obstacle{
            rotate_player(&mut player, &mut  player_direction);
        }else{
            player.row = point.row;
            player.col = point.col;
            if point.visited == false{
                visited += 1;
                point.visited = true;
                point.value = "X".to_string();
            }

        }
        
        //println!("-------------------------------------------------------------");
        //for r in &map
        //{
        //
        //    println!("{:?}",r);
        //}
        
    }
    println!("{:?}",visited);
    Ok(())
}

fn rotate_player(point :&mut Point,dir:&mut PlayerDirection)
{
    point.dx = 0;
    point.dy = 0;
    match dir {
        PlayerDirection::Up => {
            *dir = PlayerDirection::Right;
            point.dx = 1;
        }
        PlayerDirection::Down => {
            *dir = PlayerDirection::Left;
            point.dx = -1;
        }
        PlayerDirection::Left => {
            *dir = PlayerDirection::Up;
            point.dy = -1;
        }
        PlayerDirection::Right => {
            *dir = PlayerDirection::Down;
            point.dy = 1;
        }
    }

}


