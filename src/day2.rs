use std::fs;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn get_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissor => 3,
        }
    }

    fn get_wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissor,
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
        }
    }

    fn fight(&self, other: &Self) -> u32 {
        (if self == other {
            3
        } else if (&self.get_wins_against() == other) {
            6
        } else {
            0
        }) + self.get_score()
    }

    fn parse(sign: char) -> Option<Self> {
        match sign {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissor),

            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissor),
            _ => None,
        }
    }

    fn parse_with_outcome(opponent: &Self, sign: &Self) -> Option<Self> {
        match sign {
            Self::Rock => Some(opponent.get_wins_against()),
            Self::Paper => Some(opponent.clone()),
            Self::Scissor => Some(opponent.get_wins_against().get_wins_against()),
            _ => None,
        }
    }
}

pub fn day2() {
    let input = fs::read_to_string("./input/day2/input.txt").expect("Couldn't load input");

    let sum: u32 = input
        .lines()
        .map(|line| line.split(" ").take(2))
        .map(|mut splittedLine| (splittedLine.next(), splittedLine.next()))
        .map(unwrap_tuple)
        .map(curry_tuple(|str: &str| str.chars().nth(0)))
        .map(unwrap_tuple)
        .map(curry_tuple(Shape::parse))
        .map(unwrap_tuple)
        .map(|(opponent, myself)| {
            (
                opponent,
                Shape::parse_with_outcome(&opponent, &myself).unwrap(),
            )
        })
        .map(|(oponent, myself)| myself.fight(&oponent))
        .sum();

    println!("sum: {sum}")
}

fn unwrap_tuple<T>(touple: (Option<T>, Option<T>)) -> (T, T) {
    apply_tuple(touple, Option::unwrap)
}

fn apply_tuple<T, R, F: FnMut(T) -> R>((fst, snd): (T, T), mut fun: F) -> (R, R) {
    (fun(fst), fun(snd))
}

fn curry_tuple<T, R, F: FnMut(T) -> R + Copy>(fun: F) -> impl Fn((T, T)) -> (R, R) {
    move |tuple| apply_tuple(tuple, fun)
}
