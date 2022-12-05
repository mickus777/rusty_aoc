use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Assignment {
    start : u32,
    end : u32
}

#[derive(Debug)]
struct Pair {
    elf1 : Assignment,
    elf2 : Assignment
}

fn to_assignment(text : &str) -> Assignment {
    let mut ends = text.split('-');
    Assignment {
        start: ends.next().unwrap().parse().unwrap(),
        end: ends.next().unwrap().parse().unwrap()
    }
}

fn to_pair(text : &String) -> Pair {
    let mut sections = text.split(',');
    Pair {
        elf1: to_assignment(sections.next().unwrap()),
        elf2: to_assignment(sections.next().unwrap())
    }
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<Pair> {
    let mut values : Vec<Pair> = Vec::new();

    for line in lines {
        let text = line.unwrap();
        values.push(to_pair(&text));
    }

    values
}

pub fn execute(input : &String, file_name: &String) {
    let _count: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let pairs = parse_lines(lines);

    let mut count_full_contains = 0;
    let mut count_overlaps = 0;
    for pair in pairs {
        if (pair.elf1.start >= pair.elf2.start && pair.elf1.end <= pair.elf2.end) || (pair.elf2.start >= pair.elf1.start && pair.elf2.end <= pair.elf1.end) {
            count_full_contains += 1;
        }
        if (pair.elf1.start <= pair.elf2.start && pair.elf1.end >= pair.elf2.start) || (pair.elf1.start <= pair.elf2.end && pair.elf1.end >= pair.elf2.start) {
            count_overlaps += 1;
        }
    }

    println!("Full: {}", count_full_contains);
    println!("Overlaps: {}", count_overlaps);
}