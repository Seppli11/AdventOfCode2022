use std::{collections::HashSet, fs};

use itertools::Itertools;

use crate::vec::Vector;

pub fn day15() {
    let input = fs::read_to_string("./input/day15/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day15/input.txt").expect("Couldn't load input");
    let sensors = input.lines().map(Sensor::parse_line).collect_vec();
    let mut set = HashSet::new();
    let mut beacon_set = HashSet::new();
    let row = 2000000;
    for sensor in sensors {
        let range = sensor.get_range_on_row(row);
        println!("{:?}, range: {:?}", sensor, range);
        if let Some(range) = range {
            for x in range.0..=range.1 {
                if set.insert(x) {
                    //print!("{x}, ")
                }
            }
            println!()
        }
        if sensor.beacon.y == row {
            beacon_set.insert(sensor.beacon.x);
        }
    }
    let count = set.len() - beacon_set.len();
    println!("count: {count}");
}

pub fn day15b() {
    let input = fs::read_to_string("./input/day15/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day15/input.txt").expect("Couldn't load input");
    let sensors = input.lines().map(Sensor::parse_line).collect_vec();
    let max = 4000000;
    let mut pos = Vector::new(0, 0);
    for sensor in &sensors {
        println!(
            "{:?}, distance: {}",
            sensor,
            sensor.get_distance_to_beacon()
        );
    }
    loop {
        let mut found = true;
        for sensor in &sensors {
            let distance_to_sensor = sensor.pos.manhattan_distance(&pos);
            let distance_to_beacon = sensor.get_distance_to_beacon();
            if distance_to_beacon >= distance_to_sensor {
                let (_, max) = sensor.get_range_on_row(pos.y).expect("sensor is in range");
                pos.x = max;
                found = false;
                break;
            }
        }
        if found {
            println!("found pos: {pos}");
            println!("tuning freq: {}", pos.x as u64 * max as u64 + pos.y as u64);
            break;
        }

        pos.x += 1;
        if pos.x > max {
            pos.x = 0;
            pos.y += 1;
            if pos.y % 100 == 0 {
                println!("check completed: {}%", pos.y as f32 / max as f32 * 100f32);
            }
            if pos.y > max {
                panic!("y is over max {max}");
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    pos: Vector,
    beacon: Vector,
}

impl Sensor {
    fn parse_line(line: &str) -> Sensor {
        fn parse_vec(from: &str) -> Vector {
            let (x, y) = from.split_once(",").expect("invalid coordinate");
            Vector::new(
                x.parse().expect("invalid x coordinate"),
                y.parse().expect("invalid y coordinate"),
            )
        }
        let line = line
            .replace("Sensor at ", "")
            .replace("closest beacon is at ", "")
            .replace("x=", "")
            .replace("y=", "")
            .replace(" ", "");
        let (pos_str, beacon_str) = line.split_once(":").expect("no colon in line");
        Sensor {
            pos: parse_vec(pos_str),
            beacon: parse_vec(beacon_str),
        }
    }

    fn get_distance_to_beacon(&self) -> u32 {
        self.pos.manhattan_distance(&self.beacon)
    }

    fn get_range_on_row(&self, row: i32) -> Option<(i32, i32)> {
        let distance = self.get_distance_to_beacon() as i32;
        let distance = distance - self.pos.y.abs_diff(row) as i32;
        if distance <= 0 {
            None
        } else {
            let x_min = self.pos.x - distance;
            let x_max = self.pos.x + distance;
            Some((x_min, x_max))
        }
    }
}
