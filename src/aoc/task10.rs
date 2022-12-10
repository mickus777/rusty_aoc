use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let mut register_values : Vec<i32> = Vec::new();

    let mut current_value = 1;
    for line in lines {
        let text = line.unwrap();
        if text == "noop" {
            register_values.push(current_value);
        } else {
            let value = text.split(' ').nth(1).unwrap().parse::<i32>().unwrap();
            register_values.push(current_value);
            register_values.push(current_value);
            current_value += value;
        }
    }

    register_values
}

pub fn execute(input : &String, file_name: &String) {
    let _count: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let register_values = parse_lines(lines);

    let mut sum = 0;
    sum += register_values[19] * 20;
    sum += register_values[59] * 60;
    sum += register_values[99] * 100;
    sum += register_values[139] * 140;
    sum += register_values[179] * 180;
    sum += register_values[219] * 220;

    for row in 0..6 {
        for column in 0..40 {
            let cycle = row * 40 + column;
            let register_value = register_values[cycle];
            if (register_value - column as i32).abs() < 2 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("{}", sum);
}