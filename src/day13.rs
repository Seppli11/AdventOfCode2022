use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs,
    hash::Hash,
    iter::Peekable,
    num,
    rc::Rc,
    str::Chars,
};

use itertools::{EitherOrBoth, Itertools};

pub fn day13() {
    let input = fs::read_to_string("./input/day13/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day13/input.txt").expect("Couldn't load input");

    let partOne = false;
    if (partOne) {
        let packets: Vec<(Packet, Packet)> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Packet::from_str(line))
            .tuples()
            .collect();

        let mut sum = 0;
        for (i, (packet1, packet2)) in packets.iter().enumerate() {
            let order = packet1.compare_packet(packet2);
            println!("packet1: {packet1}");
            println!("packet2: {packet2}");
            println!("correct order: {:?}", order);
            println!();
            if order != Ordering::Greater {
                println!("adding {}", i + 1);
                sum += i + 1;
            }
            println!("sum: {sum}");
        }
    } else {
        let mut packets = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| Packet::from_str(line))
            .collect_vec();

        let divider_packets = vec![
            Packet::new_single_list(Packet::new_single_list(Packet::Value(2))),
            Packet::new_single_list(Packet::new_single_list(Packet::Value(6))),
        ];
        packets.append(&mut divider_packets.clone());
        packets.sort();
        let signal: usize = packets
            .iter()
            .enumerate()
            .filter(|(_, packet)| divider_packets.contains(packet))
            .map(|(i, _)| i + 1)
            .product();
        println!("distress signal: {signal}");
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(u16),
}

impl Packet {
    fn new_single_list(val: Packet) -> Self {
        Packet::List(vec![val])
    }

    fn from_str(input: &str) -> Self {
        fn parse_list(iter: &mut Peekable<impl Iterator<Item = char>>) -> Packet {
            if iter.next().expect("No bracket found") != '[' {
                panic!("no bracket found");
            }
            let mut packets = Vec::new();
            while let Some(next) = iter.peek() {
                if *next == ']' {
                    iter.next();
                    break;
                } else if *next == ',' || next.is_whitespace() {
                    iter.next();
                    continue;
                } else if next.is_alphanumeric() {
                    packets.push(parse_value(iter));
                } else if *next == '[' {
                    packets.push(parse_list(iter));
                }
            }
            Packet::List(packets)
        }

        fn parse_value(iter: &mut Peekable<impl Iterator<Item = char>>) -> Packet {
            let mut num_vec = Vec::new();
            while let Some(next) = iter.peek() {
                if next.is_alphanumeric() {
                    num_vec.push(*next);
                    iter.next();
                } else {
                    break;
                }
            }
            let num = num_vec
                .iter()
                .collect::<String>()
                .parse()
                .expect("Expected number");
            Packet::Value(num)
        }
        let mut iter = input.chars().peekable();
        parse_list(&mut iter)
    }

    fn compare_packet(&self, other: &Packet) -> Ordering {
        fn compare_list(list1: &Vec<Packet>, list2: &Vec<Packet>) -> Ordering {
            let mut iter = list1
                .iter()
                .zip_longest(list2.iter())
                .map(|pair| match pair {
                    EitherOrBoth::Both(left, right) => left.compare_packet(right),
                    EitherOrBoth::Left(_) => Ordering::Greater,
                    EitherOrBoth::Right(_) => Ordering::Less,
                })
                .peekable();
            iter.peeking_take_while(|current| *current == Ordering::Equal)
                .collect_vec();
            iter.next().unwrap_or_else(|| Ordering::Equal)
        }

        match self {
            Packet::List(self_packets) => match other {
                Packet::List(other_list) => compare_list(self_packets, other_list),
                Packet::Value(other_value) => {
                    self.compare_packet(&Packet::List(vec![Packet::Value(*other_value)]))
                }
            },
            Packet::Value(value) => match other {
                Packet::List(_) => Packet::List(vec![Packet::Value(*value)]).compare_packet(other),
                Packet::Value(other_value) => value.cmp(other_value),
            },
        }
    }

    fn first_value(&self) -> Option<u16> {
        match self {
            Packet::List(list) => list.first().and_then(|first| first.first_value()),
            Packet::Value(value) => Some(*value),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_packet(other))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::List(packets) => {
                let str_list = packets.iter().map(|packet| format!("{packet}")).join(", ");
                write!(f, "[{str_list}]")
            }
            Self::Value(value) => write!(f, "{value}"),
        }
    }
}
