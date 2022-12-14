use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Coordinate {
    x : usize,
    y : usize
}

fn to_coordinate(text : &str) -> Coordinate {
    let mut parts = text.split(',');

    Coordinate { x: parts.nth(0).unwrap().parse::<usize>().unwrap(), y: parts.nth(0).unwrap().parse::<usize>().unwrap() }
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<LinkedList<Coordinate>> {
    let mut rocks : Vec<LinkedList<Coordinate>> = Vec::new();

   for line in lines {
        let text = line.unwrap();
        let coordinates_text = text.split(" -> ");
        let mut coordinates : LinkedList<Coordinate> = LinkedList::new();
        for coordinate_text in coordinates_text {
            coordinates.push_back(to_coordinate(coordinate_text));
        }
        rocks.push(coordinates);
    }

    rocks
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Space {
    Rock,
    Sand,
    Air
}

fn print_spaces(spaces : &Vec<Vec<Space>>) {
    for y in 0..spaces[0].len() {
        for x in 0..spaces.len() {
            match spaces[x][y] {
                Space::Rock => print!("#"),
                Space::Sand => print!("o"),
                Space::Air => print!(".")
            }
        }
        println!("");
    }
}

fn mark_rock(start : &Coordinate, end : &Coordinate, spaces : &mut Vec<Vec<Space>>) {

    let mut x = start.x;
    let mut y = start.y;
    let mut delta_x = end.x as i32 - start.x as i32;
    if delta_x.abs() > 0 {
        delta_x /= delta_x.abs();
    }
    let mut delta_y = end.y as i32 - start.y as i32;
    if delta_y.abs() > 0 {
        delta_y /= delta_y.abs();
    }
    while x != end.x || y != end.y {
        spaces[x][y] = Space::Rock;
        x = (x as i32 + delta_x) as usize;
        y = (y as i32 + delta_y) as usize;
    }
    spaces[end.x][end.y] = Space::Rock;
}

fn build_spaces(rocks : &Vec<LinkedList<Coordinate>>) -> (Vec<Vec<Space>>, Coordinate) {
    let lower_x : usize = 0;
    let upper_x : usize = 1000;
    let mut upper_y : usize = 0;
    for rock in rocks {
        for node in rock {
            if node.y > upper_y {
                upper_y = node.y;
            }
        }
    }

    let mut spaces : Vec<Vec<Space>> = Vec::new();
    for _i in lower_x..(upper_x + 1) {
        spaces.push(vec![Space::Air; upper_y + 2])
    }

    for rock in rocks {
        let mut node_iter = rock.iter();
        let mut node = node_iter.next().unwrap();
        loop {
            let next_node = node_iter.next();
            if next_node.is_none() {
                break;
            }
            mark_rock(node, next_node.unwrap(), &mut spaces);
            node = next_node.unwrap();
        }
    }

    (spaces, Coordinate { x: 500, y: 0 })
}

#[derive(PartialEq)]
enum Outcome {
    Finished,
    Unfinished
}

fn move_sand(spaces : &mut Vec<Vec<Space>>, point : &Coordinate, mode : &usize) -> Outcome {

    if point.y == spaces[0].len() - 1 {
        if *mode == 1 {
            Outcome::Finished
        } else {
            spaces[point.x][point.y] = Space::Sand;
            Outcome::Unfinished
        }
    } else {
        if spaces[point.x][point.y + 1] == Space::Air {
            move_sand(spaces, &Coordinate { x: point.x, y: point.y + 1 }, mode)
        } else if spaces[point.x - 1][point.y + 1] == Space::Air {
            move_sand(spaces, &Coordinate { x: point.x - 1, y: point.y + 1 }, mode)
        } else if spaces[point.x + 1][point.y + 1] == Space::Air {
            move_sand(spaces, &Coordinate { x: point.x + 1, y: point.y + 1 }, mode)
        } else if *mode != 1 && point.x == 500 && point.y == 0 {
            spaces[point.x][point.y] = Space::Sand;
            Outcome::Finished
        } else {
            spaces[point.x][point.y] = Space::Sand;
            Outcome::Unfinished
        }
    }
}

fn fill_sand(spaces : &mut Vec<Vec<Space>>, start_point : &Coordinate, mode : usize) {
    while move_sand(spaces, start_point, &mode) == Outcome::Unfinished {}
}

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let rocks = parse_lines(lines);
    let (mut spaces, start_point) = build_spaces(&rocks);

    fill_sand(&mut spaces, &start_point, mode);

    print_spaces(&spaces);

    let mut count = 0;
    for y in 0..spaces[0].len() {
        for x in 0..spaces.len() {
            if spaces[x][y] == Space::Sand {
                count += 1;
            }
        }
    }

    println!("{}", count);
}