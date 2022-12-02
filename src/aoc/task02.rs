use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissor
}

#[derive(PartialEq)]
enum Outcome {
    Win, 
    Draw,
    Loss
}

fn to_shape(name : &str) -> Shape {
    match name {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissor,
        _ => Shape::Rock
    }
}

fn to_outcome(name : &str) -> Outcome {
    match name {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => Outcome::Draw
    }
}

fn calculate_outcome(me : &Shape, opponent : &Shape) -> Outcome {
    if me == opponent {
        Outcome::Draw
    } else {
        match me {
            Shape::Rock => if *opponent == Shape::Paper { Outcome::Loss } else { Outcome::Win },
            Shape::Paper => if *opponent == Shape::Scissor { Outcome::Loss } else { Outcome::Win },
            Shape::Scissor => if *opponent == Shape::Rock { Outcome::Loss } else { Outcome::Win }
        }
    }
}

fn calculate_shape_choice(shape : &Shape, outcome : Outcome) -> Shape {
    if outcome == Outcome::Draw {
        match shape {
            Shape::Rock => Shape::Rock,
            Shape::Paper => Shape::Paper,
            Shape::Scissor => Shape::Scissor,
        }
    } else {
        match shape {
            Shape::Rock => if outcome == Outcome::Win { Shape::Paper } else { Shape::Scissor },
            Shape::Paper => if outcome == Outcome::Win { Shape::Scissor } else { Shape::Rock },
            Shape::Scissor => if outcome == Outcome::Win { Shape::Rock } else { Shape::Paper },
        }
    }
}

fn score_shape(shape : Shape) -> u32 {
    match shape {
        Shape::Paper => 2,
        Shape::Rock => 1,
        Shape::Scissor => 3
    }
}

fn score_outcome(outcome : Outcome) -> u32 {
    match outcome {
        Outcome::Draw => 3,
        Outcome::Loss => 0,
        Outcome::Win => 6
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>, mode : usize) -> Vec<u32> {
    let mut values : Vec<u32> = Vec::new();

    for line in lines {
        let text = line.unwrap();
        let mut strategy = text.split(' ');
        let opponent = strategy.next().unwrap();
        let instruction = strategy.next().unwrap();

        let my_shape: Shape;
        let opponent_shape = to_shape(opponent);
        if mode == 1 {
            my_shape = to_shape(instruction);
        } else {
            let necessary_outcome = to_outcome(instruction);
            my_shape = calculate_shape_choice(&opponent_shape, necessary_outcome);
        }
        let outcome = calculate_outcome(&my_shape, &opponent_shape);

        let value = score_shape(my_shape) + score_outcome(outcome);

        values.push(value);
    }

    values
}

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let values = parse_lines(lines, mode);

    let sum : u32 = values.iter().sum();

    println!("{}", sum);
}