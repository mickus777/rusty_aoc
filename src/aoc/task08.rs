use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<Vec<u32>> {
    let mut grid : Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let mut row = Vec::new();
        for ch in line.unwrap().chars() {
            row.push(ch.to_digit(10).unwrap());
        }
        grid.push(row);
    }

    grid
}

fn mark_visibility(grid : &Vec<Vec<u32>>, visibility : &mut Vec<Vec<bool>>, start_row : i32, start_column : i32, delta_row : i32, delta_column : i32) {

    let mut row = start_row;
    let mut column = start_column;

    let mut height = 0;

    loop {
        if grid[row as usize][column as usize] > height {
            visibility[row as usize][column as usize] = true;
            height = grid[row as usize][column as usize];
        }

        row += delta_row;
        column += delta_column;
        if delta_row != 0 && (row < 1 || row > (grid.len() - 1) as i32) {
            break;
        }
        if delta_column != 0 && (column < 1 || column > (grid.len() - 1) as i32) {
            break;
        }
    }
}

fn create_visibilty_grid(grid : &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visibility = Vec::new();

    for row in grid.iter() {
        visibility.push(vec![false; row.len()]);
    }
    
    for index in 1..(grid.len() - 1) {
        mark_visibility(&grid, &mut visibility, index as i32, 0, 0, 1);
        mark_visibility(&grid, &mut visibility, index as i32, (grid.len() - 1) as i32, 0, -1);
        mark_visibility(&grid, &mut visibility, 0, index as i32, 1, 0);
        mark_visibility(&grid, &mut visibility, (grid.len() - 1) as i32, index as i32, -1, 0);
    }
    for index in 0..grid.len() {
        visibility[0][index] = true;
        visibility[grid.len() - 1][index] = true;
        visibility[index][0] = true;
        visibility[index][grid.len() - 1] = true;
    }

    visibility
}

fn calculate_view_distance(grid : &Vec<Vec<u32>>, mut row : usize, mut column : usize, delta_row : i32, delta_column : i32, height : u32) -> u64 {

    let mut count = 0;

    loop {
        if delta_row < 0 && row == 0 || delta_row > 0 && row == grid.len() - 1 || delta_column < 0 && column == 0 || delta_column > 0 && column == grid.len() - 1 {
            break;
        }

        row = (row as i32 + delta_row) as usize;
        column = (column as i32 + delta_column) as usize;

        count += 1;
        
        if grid[row][column] >= height {
            break;
        }
    }

    count
}

fn calculate_score(grid : &Vec<Vec<u32>>, row : usize, column : usize) -> u64 {

    let mut score = 1;

    score *= calculate_view_distance(grid, row, column, 1, 0, grid[row][column]);
    score *= calculate_view_distance(grid, row, column, -1, 0, grid[row][column]);
    score *= calculate_view_distance(grid, row, column, 0, 1, grid[row][column]);
    score *= calculate_view_distance(grid, row, column, 0, -1, grid[row][column]);

    score
}

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let grid = parse_lines(lines);

    if mode == 1 {
        let visibility_grid = create_visibilty_grid(&grid);

        let mut count = 0;
    
        for row in 0..(grid.len()) {
            for column in 0..(grid.len()) {
                if visibility_grid[row][column] {
                    count += 1;
                }
            }
        }
    
        println!("{}", count);
    } else if mode == 2 {
        let mut best_score = 0;

        for row in 0..(grid.len()) {
            for column in 0..(grid.len()) {
                let score = calculate_score(&grid, row, column);
                if score > best_score {
                    best_score = score;
                }
            }
        }

        println!("{}", best_score);
    }
}