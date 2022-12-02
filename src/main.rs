use std::env;

pub mod aoc;

fn main() {
    let args: Vec<String> = env::args().collect();

    let task_no: i32 = args[1].parse().unwrap();
    let input = &args[2];
    let file_name = &args[3];
    match task_no {
        1 => aoc::task01::execute(input, file_name),
        2 => aoc::task02::execute(input, file_name),
        _ => println!("Unknown task"),
    }
}
