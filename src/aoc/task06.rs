use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::LinkedList;
use std::collections::HashSet;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn check_unique_window(window : &LinkedList<char>, count : usize) -> bool {
    let mut set : HashSet<char> = HashSet::new();

    for ch in window {
        set.insert(*ch);
    }

    set.len() == count
}

pub fn execute(input : &String, file_name: &String) {
    let count: usize = input.parse().unwrap();

    let line = read_lines(file_name).unwrap().next().unwrap().unwrap();

    let mut window : LinkedList<char> = LinkedList::new();
    let mut index = 0;
    for ch in line.chars() {
        index += 1;
        if window.len() > count - 1 {
            window.pop_front();
        }
        window.push_back(ch);
        if check_unique_window(&window, count) {
            break;
        }
    }

    println!("{}", index);
}