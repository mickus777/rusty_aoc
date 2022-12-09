use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

fn to_direction(ch : &char) -> Direction {
    match ch {
        'R' => Direction::Right,
        'L' => Direction::Left,
        'U' => Direction::Up,
        'D' => Direction::Down,
        _ => Direction::Down
    }
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<Direction> {
    let mut instructions : Vec<Direction> = Vec::new();

    for line in lines {
        let text = line.unwrap();
        let mut parts = text.split(' ');
        let direction = parts.next().unwrap().chars().nth(0).unwrap();
        let times = parts.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..times {
            instructions.push(to_direction(&direction));
        }
    }

    instructions
}

struct Coordinate {
    x : i32,
    y : i32
}

fn move_direction(coordinate : &mut Coordinate, direction : &Direction) {
    match direction {
        Direction::Down => coordinate.y -= 1,
        Direction::Up => coordinate.y += 1,
        Direction::Left => coordinate.x -= 1,
        _ => coordinate.x += 1
    }
}

fn follow_previous(previous_x : &i32, previous_y : &i32, knot : &mut Coordinate) {
    let delta_x = previous_x - knot.x;
    let delta_y = previous_y - knot.y;
    let x_distance = delta_x.abs();
    let y_distance = delta_y.abs();
    if x_distance > 0 && y_distance > 0 && (x_distance > 1 || y_distance > 1) {
        if delta_y > 0 {
            knot.y += 1;
        } else {
            knot.y -= 1;
        }
        if delta_x > 0 {
            knot.x += 1;
        } else {
            knot.x -= 1;
        }
    } else if x_distance > 1 {
        if delta_x > 0 {
            knot.x += 1;
        } else {
            knot.x -= 1;
        }
    } else if y_distance > 1 {
        if delta_y > 0 {
            knot.y += 1;
        } else {
            knot.y -= 1;
        }
    } 
}

pub fn execute(input : &String, file_name: &String) {
    let length: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let instructions = parse_lines(lines);

    let mut visited_positions : HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut knots : Vec<Coordinate> = Vec::new();
    for _ in 0..length {
        knots.push(Coordinate { x: 0, y: 0 });
    }

    for instruction in instructions {
        if !visited_positions.contains_key(&knots.last().unwrap().x) {
            visited_positions.insert(knots.last().unwrap().x, HashSet::new());
        }
        visited_positions.get_mut(&knots.last().unwrap().x).unwrap().insert(knots.last().unwrap().y);

        move_direction(knots.first_mut().unwrap(), &instruction);
        for i in 1..(knots.len()) {
            let previous = knots.get(i - 1).unwrap();
            let previous_x = previous.x;
            let previous_y = previous.y;
            follow_previous(&previous_x, &previous_y, knots.get_mut(i).unwrap());
        }
    }
    if !visited_positions.contains_key(&knots.last().unwrap().x) {
        visited_positions.insert(knots.last().unwrap().x, HashSet::new());
    }
    visited_positions.get_mut(&knots.last().unwrap().x).unwrap().insert(knots.last().unwrap().y);

    let mut count = 0;
    for x in visited_positions {
        count += x.1.len();
    }

    println!("{}", count);
}