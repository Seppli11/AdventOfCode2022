use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    fs,
    hash::Hash,
    rc::Rc,
};

use itertools::Itertools;

pub fn day12() {
    let input = fs::read_to_string("./input/day12/test-input.txt").expect("Couldn't load input");
    let input = fs::read_to_string("./input/day12/input.txt").expect("Couldn't load input");
    let map = Map::from_string_map(&input);
    let pos = Pos::new(0, 0);

    let start = map.find_end().unwrap();
    let end = map.find_start().unwrap();
    start.borrow_mut().cost = 0;

    let mut queue = VecDeque::new();
    queue.push_back(start.clone());
    let mut visited: HashSet<Pos> = HashSet::new();
    while !queue.is_empty() {
        let current_cell = queue.pop_front().unwrap();
        let current_ref = current_cell.borrow();
        // skip if already visited
        if visited.contains(&current_ref.position) {
            println!("skip {}", current_ref.position);
            continue;
        }
        //println!("process {}", current_ref.position);

        visited.insert(current_ref.position);

        for neighbor in &current_ref.neighbors {
            if current_ref.cost + 1 < neighbor.borrow().cost
                //&& current_ref.height.can_walk_to(&neighbor.borrow().height)
                && neighbor.borrow().height.can_walk_to(&current_ref.height)
            {
                let mut neighbor_ref = neighbor.borrow_mut();
                neighbor_ref.cost = current_ref.cost + 1;
                neighbor_ref.last.replace(current_cell.clone());
                queue.push_back(neighbor.clone());
            }
        }
    }
    println!("done");
    let all_starts = map.find_zero_elevation_tiles();
    let start = all_starts[0];
    println!(
        "end cost: {}, end pos: {}",
        start.borrow().cost,
        start.borrow().position
    );
    let mut current = end.clone();
    while current.borrow().height != Height::Start {
        let next = current.borrow().last.clone().unwrap();
        println!(
            "pos: {}, height: {}",
            current.borrow().position,
            current.borrow().height
        );
        current = next.clone();
    }
}

type RcCell<T> = Rc<RefCell<T>>;
type TileRef = RcCell<Tile>;

struct Tile {
    position: Pos,
    height: Height,
    neighbors: Vec<TileRef>,
    last: Option<TileRef>,
    cost: u32,
}

impl Tile {
    fn new(position: Pos, height: char) -> Tile {
        Tile {
            position,
            height: Height::new(height),
            neighbors: vec![],
            last: None,
            cost: u32::MAX,
        }
    }

    fn add_neighbor(&mut self, tile: TileRef) {
        if let None = self
            .neighbors
            .iter()
            .find(|neighbor| neighbor.borrow().position == tile.borrow().position)
        {
            self.neighbors.push(tile);
        }
    }
}

struct Map {
    tiles: HashMap<Pos, TileRef>,
}

impl Map {
    fn from_string_map(lines: &str) -> Map {
        let mut map = Map {
            tiles: HashMap::new(),
        };
        lines
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, char)| {
                    (
                        Pos {
                            x: x as u32,
                            y: y as u32,
                        },
                        char,
                    )
                })
            })
            .map(|(pos, height)| Tile::new(pos, height))
            .for_each(|tile| map.add_tile(tile));
        map
    }

    fn add_tile(&mut self, tile: Tile) {
        let position = tile.position;
        let tile = RcCell::new(RefCell::new(tile));
        self.tiles.insert(position, tile.clone());

        for neighbor_pos in position.get_neighbors() {
            if let Some(neighbor_tile) = self.tiles.get(&neighbor_pos) {
                neighbor_tile.borrow_mut().add_neighbor(tile.clone());
                tile.borrow_mut().add_neighbor(neighbor_tile.clone());
            }
        }
    }

    fn get_tile(&self, pos: Pos) -> Option<&TileRef> {
        self.tiles.get(&pos)
    }

    fn find_start(&self) -> Option<&TileRef> {
        self.tiles
            .values()
            .find(|tile| tile.borrow().height == Height::Start)
    }

    fn find_end(&self) -> Option<&TileRef> {
        self.tiles
            .values()
            .find(|tile| tile.borrow().height == Height::End)
    }

    fn find_zero_elevation_tiles(&self) -> Vec<&TileRef> {
        self.tiles
            .values()
            .filter(|tile| tile.borrow().height.get_height() == 0)
            .sorted_by_key(|tile| tile.borrow().cost)
            .collect_vec()
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn new(x: u32, y: u32) -> Pos {
        Pos { x, y }
    }

    fn get_neighbors(&self) -> Vec<Pos> {
        let mut neighbors = vec![
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
        ];
        if self.x > 0 {
            neighbors.push(Pos {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            neighbors.push(Pos {
                x: self.x,
                y: self.y - 1,
            });
        }

        neighbors
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{})", self.x, self.y)
    }
}

#[derive(PartialEq)]
enum Height {
    Start,
    End,
    Height(u32),
}

impl Height {
    fn new(char: char) -> Height {
        match char {
            'S' => Height::Start,
            'E' => Height::End,
            _ => Height::Height(char as u32 - 'a' as u32),
        }
    }

    fn get_height(&self) -> u32 {
        match self {
            Height::Start => 0,
            Height::End => 'z' as u32 - 'a' as u32,
            Height::Height(self_height) => *self_height,
        }
    }

    fn can_walk_to(&self, other: &Self) -> bool {
        other.get_height() as i64 - self.get_height() as i64 <= 1
    }
}

impl Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Height::Start => f.write_str("S"),
            Height::End => f.write_str("E"),
            Height::Height(height) => write!(f, "{height}"),
        }
    }
}
