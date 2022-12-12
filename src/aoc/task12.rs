use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
    let mut map : Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let text = line.unwrap();
        let mut row = Vec::new();
        for ch in text.chars() {
            row.push(ch as u32);
        }
        map.push(row);
    }

    map
}

fn find_location(ch : u32, map : &Vec<Vec<u32>>) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0] .len() {
            if map[y][x] == ch {
                return (x, y)
            }            
        }
    }

    (0, 0)
}

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let mut map = parse_lines(lines);
    let end = find_location('E' as u32, &map);

    let mut distances : Vec<Vec<usize>> = Vec::new();
    for _i in 0..map.len() {
        distances.push(vec![map.len() * map[0].len() + 97; map[0].len()]);
    }

    let mut positions : LinkedList<(usize, usize)> = LinkedList::new();
    let start = find_location('S' as u32, &map);
    map[start.1][start.0] = 97;
    distances[start.1][start.0] = 0;
    positions.push_back(start);
    if mode == 2 {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 97 {
                    distances[y][x] = 0;
                    positions.push_back((x, y));
                }
            }
        }
    }
    map[end.1][end.0] = 122;
    while positions.len() > 0 {
        let position = positions.pop_front().unwrap();
        if position.0 == end.0 && position.1 == end.1 {
            break;
        }

        let height = map[position.1][position.0];
        let distance = distances[position.1][position.0];
        if position.0 > 0 {
            if map[position.1][position.0 - 1] <= height + 1 && distance + 1 < distances[position.1][position.0 - 1] {
                distances[position.1][position.0 - 1] = distance + 1;
                positions.push_back((position.0 - 1, position.1));
            }
        } 
        if position.0 < map[0].len() - 1 {
            if map[position.1][position.0 + 1] <= height + 1 && distance + 1 < distances[position.1][position.0 + 1] {
                distances[position.1][position.0 + 1] = distance + 1;
                positions.push_back((position.0 + 1, position.1));
            }
        }
        if position.1 > 0 {
            if map[position.1 - 1][position.0] <= height + 1 && distance + 1 < distances[position.1 - 1][position.0] {
                distances[position.1 - 1][position.0] = distance + 1;
                positions.push_back((position.0, position.1 - 1));
            }
        } 
        if position.1 < map.len() - 1 {
            if map[position.1 + 1][position.0] <= height + 1 && distance + 1 < distances[position.1 + 1][position.0] {
                distances[position.1 + 1][position.0] = distance + 1;
                positions.push_back((position.0, position.1 + 1));
            }
        }
    }

    println!("{}", distances[end.1][end.0]);
}