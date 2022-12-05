use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct MoveInstruction {
    count: usize,
    from: usize,
    to: usize
}

#[derive(Debug)]
struct Problem {
    stacks : Vec<Vec<char>>,
    instructions : Vec<MoveInstruction>
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Problem {

    let mut stacks_done = false;
    let mut stack_lines : Vec<String> = Vec::new();
    let mut instruction_lines : Vec<String> = Vec::new();
    for line in lines {
        let text = line.unwrap();
        if text.len() < 1 {
            stacks_done = true;
        } else if !stacks_done {
            stack_lines.push(text);
        } else {
            instruction_lines.push(text);
        }
    }

    let mut stacks = Vec::new();
    for stack_line in stack_lines.iter().rev() {
        let mut index : usize = 0;
        while 1 + index * 4 < stack_line.len() {
            if stacks.len() < index + 1 {
                stacks.push(Vec::new());
            } else {
                let ch = stack_line.chars().nth(1 + index * 4).unwrap();
                if ch != ' ' {
                    stacks[index].push(ch);
                }
            }
            index += 1;
        }
    }

    let mut instructions = Vec::new();
    for instruction_line in instruction_lines {
        let mut segments = instruction_line.split(' ');
        let count = segments.nth(1).unwrap().parse().unwrap();
        let from = segments.nth(1).unwrap().parse().unwrap();
        let to = segments.nth(1).unwrap().parse().unwrap();
        instructions.push(MoveInstruction { count: count, from: from, to: to })
    }

    return Problem { stacks: stacks, instructions: instructions }
}

fn execute_instruction_9000(stacks : &mut Vec<Vec<char>>, instruction : &MoveInstruction) {
    for _times in 0..instruction.count {
        let ch = stacks[instruction.from - 1].pop().unwrap();
        stacks[instruction.to - 1].push(ch);
    }
}

fn execute_instruction_9001(stacks : &mut Vec<Vec<char>>, instruction : &MoveInstruction) {
    let mut temp = Vec::new();

    for _times in 0..instruction.count {
        let ch = stacks[instruction.from - 1].pop().unwrap();
        temp.push(ch);
    }

    for _times in 0..instruction.count {
        let ch = temp.pop().unwrap();
        stacks[instruction.to - 1].push(ch);
    }
}

pub fn execute(input : &String, file_name: &String) {
    let model: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let mut problem = parse_lines(lines);

    for instruction in problem.instructions.iter() {
        match model {
            9000 => execute_instruction_9000(&mut problem.stacks, &instruction),
            9001 => execute_instruction_9001(&mut problem.stacks, &instruction),
            _ => {}
        }
    }

    for mut stack in problem.stacks {
        print!("{}", stack.pop().unwrap());
    }

    println!("");
}