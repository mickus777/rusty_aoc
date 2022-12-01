use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<u32> {
    let mut values : Vec<u32> = Vec::new();

    let mut current_value = 0;
    for line in lines {
        let text = line.unwrap();
        if text.len() < 1 {
            values.push(current_value);
            current_value = 0;
        } else {
            current_value += text.parse::<u32>().unwrap();
        }
    }
    values.push(current_value);

    values
}

pub fn execute(input : &String, file_name: &String) {
    let count: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let mut values = parse_lines(lines);
    values.sort();
    values.reverse();

    let mut sum = 0;
    for index in 0..count {
        sum += values[index];
    }

    println!("{}", sum);
}