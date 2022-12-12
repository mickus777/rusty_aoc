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
        3 => aoc::task03::execute(input, file_name),
        4 => aoc::task04::execute(input, file_name),
        5 => aoc::task05::execute(input, file_name),
        6 => aoc::task06::execute(input, file_name),
        7 => aoc::task07::execute(input, file_name),
        8 => aoc::task08::execute(input, file_name),
        9 => aoc::task09::execute(input, file_name),
        10 => aoc::task10::execute(input, file_name),
        11 => aoc::task11::execute(input, file_name),
        12 => aoc::task12::execute(input, file_name),
        _ => println!("Unknown task"),
    }
}
