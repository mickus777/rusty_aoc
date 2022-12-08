use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct DeviceFile {
    size : u64
}

#[derive(Debug)]
struct DeviceFolder {
    name : String,
    sub_folders : Vec<DeviceFolder>,
    files : Vec<DeviceFile>
}

fn get_folder<'a>(path : &Vec<String>, root : &'a mut DeviceFolder) -> &'a mut DeviceFolder {
    let mut current_folder = root;

    for dir in path {
        for index in 0..current_folder.sub_folders.len() {
            if current_folder.sub_folders[index].name == *dir {
                current_folder = &mut current_folder.sub_folders[index];
                break;
            }
        }
    }

    current_folder
}

fn parse_lines<'a>(lines : io::Lines<io::BufReader<File>>) -> DeviceFolder {
    let mut root = DeviceFolder { 
        name : "/".to_string(), 
        sub_folders : Vec::new(), 
        files : Vec::new() 
    };

    let mut path : Vec<String> = Vec::new();

    for line in lines {
        let text = line.unwrap();
        let parts : Vec<&str> = text.split(' ').collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == "/" {
                    path.clear();
                } else if parts[2] == ".." {
                    path.pop();
                } else {
                    path.push(parts[2].to_string());
                }
            } else {
                // ls, ignore
            }
        } else {
            let folder = get_folder(&path, &mut root);
            if parts[0] == "dir" {
                folder.sub_folders.push(DeviceFolder {
                    name : parts[1].to_string(),
                    sub_folders : Vec::new(),
                    files : Vec::new()
                })
            } else {
                folder.files.push(DeviceFile {
                    size : parts[0].parse::<u64>().unwrap()
                })
            }
        }
    }

    root
}

fn get_folder_sizes(folder : &DeviceFolder) -> Vec<u64> {
    let mut size = 0;

    let mut sizes : Vec<u64> = Vec::new();

    for file in &folder.files {
        size += file.size;
    }
    for sub_folder in &folder.sub_folders {
        let mut sub_sizes = get_folder_sizes(sub_folder);
        sizes.append(&mut sub_sizes);
        size += sizes.last().unwrap();
    }

    sizes.push(size);

    sizes
}

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let root = parse_lines(lines);

    let mut sizes = get_folder_sizes(&root);

    let mut sum = 0;

    if mode == 1 {
        for size in sizes {
            if size <= 100000 {
                sum += size;
            }
        }
    } else if mode == 2 {
        sizes.sort();
        for size in sizes {
            if size >= 8381165 {
                sum += size;
                break;
            }
        }
    }

    println!("{}", sum);
}