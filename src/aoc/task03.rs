use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::Chars;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_chars(set : &mut HashSet<char>, text : Chars) {
    for ch in text {
        set.insert(ch);
    }
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>, mode : usize) -> Vec<char> {
    let mut values : Vec<char> = Vec::new();

    if mode == 1 {
        for line in lines {
            let mut first : HashSet<char> = HashSet::new();
            let mut second : HashSet<char> = HashSet::new();
            
            let text = line.unwrap();
            read_chars(&mut first, text[0..(text.len() / 2)].chars());
            read_chars(&mut second, text[(text.len() / 2)..].chars());

            values.push(*(first.intersection(&second).next().unwrap()));
        }
    } else {
        let mut first : HashSet<char> = HashSet::new();
        let mut second : HashSet<char> = HashSet::new();
        let mut third : HashSet<char> = HashSet::new();
        for line in lines {
            if first.len() == 0 {
                read_chars(&mut first, line.unwrap().chars());
            } else if second.len() == 0 {
                read_chars(&mut second, line.unwrap().chars());
            } else {
                read_chars(&mut third, line.unwrap().chars());

                for item in first.intersection(&second) {
                    if third.contains(item) {
                        values.push(*item);
                        break;
                    }
                }
                first.clear();
                second.clear();
                third.clear();
            }
        }
    }

    values
}

fn calculate_item_value(ch : char) -> u32 {
    let pos = ch as u32;
    if pos < 97 {
        pos - 64 + 26
    } else {
        pos - 96
    }
}

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let values = parse_lines(lines, mode);

    let mut sum = 0;
    for ch in values {
        sum += calculate_item_value(ch);
    }

    println!("{}", sum);
}