use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum Token {
    Start,
    End,
    Value(u32)
}

fn tokenize(line : &String) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut number = String::new();

    for ch in line.chars() {
        if ch == ']' {
            if number.len() > 0 {
                tokens.push(Token::Value(number.parse::<u32>().unwrap()));
                number.clear();
            }
            tokens.push(Token::End);
        } else if ch == '[' {
            tokens.push(Token::Start);
        } else if ch == ',' {
            if number.len() > 0 {
                tokens.push(Token::Value(number.parse::<u32>().unwrap()));
                number.clear();
            }
        } else {
            number.push(ch);
        }
    }

    tokens
}

#[derive(Debug)]
enum Packet {
    Value(u32),
    List(Vec<Packet>)
}

fn packetize(tokens : &Vec<Token>, index : &mut usize) -> Packet {
    let mut child_packets : Vec<Packet> = Vec::new();

    // Peel of initial '['
    *index += 1;

    loop {
        match tokens[*index] {
            Token::Start => child_packets.push(packetize(tokens, index)),
            Token::End => {
                *index += 1;
                return Packet::List(child_packets);
            },
            Token::Value(n) => {
                child_packets.push(Packet::Value(n));
                *index += 1;
            }
        }
    }
}

fn parse_line(line : &String) -> Packet {
    let tokens = tokenize(line);
    let mut index = 0;
    packetize(&tokens, &mut index)
}

fn parse_lines(lines : io::Lines<io::BufReader<File>>) -> Vec<(Packet, Packet)> {
    let mut values : Vec<(Packet, Packet)> = Vec::new();

    let mut value1 = Option::None;

    for line in lines {
        let text = line.unwrap();
        if text.len() < 1 {
            // Do nothing
        } else if value1.is_none() {
            value1 = Option::Some(parse_line(&text));
        } else {
            values.push((value1.unwrap(), parse_line(&text)));
            value1 = Option::None;
        }
    }

    values
}

impl Ord for Packet {
    fn cmp(&self, p2 : &Packet) -> Ordering {
        if let Packet::Value(v1) = self {
            if let Packet::Value(v2) = p2 {
                if v1 < v2 {
                    return Ordering::Less;
                } else if v1 > v2 {
                    return Ordering::Greater;
                } else {
                    return Ordering::Equal;
                }
            } else {
                let vec1 = vec![Packet::Value(*v1)];
                let r = Packet::List(vec1).cmp(p2);
                if r != Ordering::Equal {
                    return r;
                }
            }
        } else if let Packet::List(l1) = self {
            if let Packet::List(l2) = p2 {
                for index in 0..l1.len() {
                    if index >= l2.len() {
                        return Ordering::Greater;
                    } else {
                        let r = l1[index].cmp(&l2[index]);
                        if r != Ordering::Equal {
                            return r;
                        }
                    }
                }
                if l1.len() < l2.len() {
                    return Ordering::Less;
                }
            } else if let Packet::Value(v2) = p2 {
                let vec2 = vec![Packet::Value(*v2)];
                let r = self.cmp(&Packet::List(vec2));
                if r != Ordering::Equal {
                    return r;
                }
            }
        }
    
        Ordering::Equal
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet { }

pub fn execute(input : &String, file_name: &String) {
    let mode: usize = input.parse().unwrap();

    let lines = read_lines(file_name).unwrap();
    let packets = parse_lines(lines);

    let mut index = 1;
    let mut sum = 0;
    if mode == 1 {
        for packet_pair in packets {
            if packet_pair.0.cmp(&packet_pair.1) != Ordering::Greater {
                sum += index;
            }
            index += 1;
        }
    } else {
        let mut all_packets = Vec::new();
        for packet in packets {
            all_packets.push(packet.0);
            all_packets.push(packet.1);
        }
        all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(2)])]));
        all_packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(6)])]));

        all_packets.sort();

        let point1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
        let point2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
        for packet in all_packets {
            if packet.cmp(&point1) == Ordering::Equal {
                sum = index;
            } else if packet.cmp(&point2) == Ordering::Equal {
                sum *= index;
            }
            index += 1;
        }
    }

    println!("{}", sum);
}