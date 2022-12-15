use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Coordinate {
    x : i64,
    y : i64
}

#[derive(Debug)]
struct SensorData {
    location : Coordinate,
    beacon : Coordinate
}

fn parse_number(text : &str, suffix : usize) -> i64 {
    text[2..(text.len() - suffix)].parse::<i64>().unwrap()
}

fn parse_sensor(text : &String) -> SensorData {
    let mut parts = text.split(' ');

    let sensor_x = parse_number(parts.nth(2).unwrap(), 1);
    let sensor_y = parse_number(parts.nth(0).unwrap(), 1);
    let beacon_x = parse_number(parts.nth(4).unwrap(), 1);
    let beacon_y = parse_number(parts.nth(0).unwrap(), 0);

    SensorData { location: Coordinate { x: sensor_x, y: sensor_y }, beacon: Coordinate { x: beacon_x, y: beacon_y } }
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<SensorData> {
    let mut sensors : Vec<SensorData> = Vec::new();

    for line in lines {
        let text = line.unwrap();
        sensors.push(parse_sensor(&text));
    }

    sensors
}

fn manhattan_distance(c1 : &Coordinate, c2 : &Coordinate) -> i64 {
    (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
}

#[derive(Debug)]
struct Range {
    lower : i64,
    upper : i64
}

fn merge_ranges(mut ranges : Vec<Range>) -> Vec<Range> {
    ranges.sort_by_key(|r| r.lower);
    ranges.reverse();

    let mut last = ranges.pop().unwrap();

    let mut merged : Vec<Range> = Vec::new();

    while ranges.len() > 0 {
        let range = ranges.pop().unwrap();
        if range.lower <= last.upper {
            if range.upper > last.upper {
                last.upper = range.upper;
            }
        } else {
            merged.push(last);
            last = range;
        }
    }
    merged.push(last);

    merged
}

fn find_overlap_ranges(sensors : &Vec<SensorData>, row : i64) -> Vec<Range>{
    let mut ranges : Vec<Range> = Vec::new();

    for sensor in sensors.iter() {
        let distance = manhattan_distance(&sensor.location, &sensor.beacon);
        if distance < (sensor.location.y - row).abs() {
            continue;
        } else {
            let impact = distance - (sensor.location.y - row).abs();
            let range = Range { lower: sensor.location.x - impact, upper: sensor.location.x + impact };
            ranges.push(range);    
        }
    }

    merge_ranges(ranges)
}

pub fn execute(input : &String, file_name: &String) {
    let row: i64 = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let sensors = parse_lines(lines);

    let ranges = find_overlap_ranges(&sensors, row);

    let mut count = 0;
    for range in ranges.iter() {
        count += range.upper - range.lower;
    }

    let mut frequency = 0;
    for scan in 0..(row * 2) {
        let scan_ranges = find_overlap_ranges(&sensors, scan);
        if scan_ranges.len() > 1 {
            frequency = (scan_ranges[0].upper + 1) * 4000000 + scan;
            break;
        }
    }

    println!("{} {}", count, frequency);
}