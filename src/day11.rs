use std::{collections::VecDeque, fmt::Debug};

use itertools::Itertools;

pub fn day11() {
    let mut group = MonkeyGroup::create_test_input();
    let mut group = MonkeyGroup::create_input();
    for i in 0..10000 {
        group.inspect_all();
        //println!("iteration {i}");
        //println!("{:?}", group);
        /*println!(
            "inspection counts: {:?}",
            group
                .inspection_counters()
                //.sorted_by(|x, y| y.cmp(x))
                .collect_vec()
        );*/
    }

    let monkey_business_level: u64 = group
        .inspection_counters()
        .sorted_by(|x, y| y.cmp(x))
        .take(2)
        .product();
    println!("monkey business level: {monkey_business_level}");
    println!(
        "inspection counts: {:?}",
        group
            .inspection_counters()
            .sorted_by(|x, y| y.cmp(x))
            .collect_vec()
    )
}

struct Monkey {
    nr: u32,
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    divisible_by: u64,
    true_throw: u32,
    false_throw: u32,
    inspection_count: u64,
}

impl Monkey {
    fn inspect(&mut self, modulo: u64) -> Inspect<'_> {
        Inspect {
            monkey: self,
            modulo,
        }
    }
}

struct Inspect<'a> {
    monkey: &'a mut Monkey,
    modulo: u64,
}

impl<'a> Iterator for Inspect<'a> {
    type Item = (u32, u64);

    fn next(&mut self) -> Option<(u32, u64)> {
        let item = self.monkey.items.pop_front()?;
        let worry_level = (self.monkey.operation)(item) % self.modulo;

        let next_monkey = if worry_level % self.monkey.divisible_by == 0 {
            self.monkey.true_throw
        } else {
            self.monkey.false_throw
        };

        self.monkey.inspection_count += 1;

        /*            println!(
            "move item {item} (worry level: {worry_level}) from {} to {next_monkey} (inspection_counter: {})",
            self.monkey.nr, self.monkey.inspection_count
        );*/

        Some((next_monkey, worry_level))
    }
}

struct MonkeyGroup(Vec<Monkey>);

impl MonkeyGroup {
    fn inspect_all(&mut self) {
        let modulo = self.calculate_modulo();
        for i in 0..self.0.len() {
            let monkey = self.0.get_mut(i).unwrap();
            let moves = monkey.inspect(modulo).collect_vec();

            for (new_monkey, item) in moves {
                let new_monkey = self.find_monkey_mut(new_monkey).unwrap();
                new_monkey.items.push_back(item);
            }
        }
    }

    fn calculate_modulo(&self) -> u64 {
        self.0.iter().map(|monkey| monkey.divisible_by).product()
    }

    fn find_monkey_mut(&mut self, nr: u32) -> Option<&mut Monkey> {
        self.0.iter_mut().find(|monkey| monkey.nr == nr)
    }

    fn inspection_counters(&self) -> impl Iterator<Item = u64> + '_ {
        self.0.iter().map(|monkey| monkey.inspection_count)
    }
}

impl MonkeyGroup {
    fn create_test_input() -> MonkeyGroup {
        let monkeys = vec![
            Monkey {
                nr: 0,
                items: VecDeque::from(vec![79, 98]),
                //operation: |old| old.wrapping_mul(19),
                operation: |old| old * 19,
                divisible_by: 23,
                true_throw: 2,
                false_throw: 3,
                inspection_count: 0,
            },
            Monkey {
                nr: 1,
                items: VecDeque::from(vec![54, 65, 75, 74]),
                //operation: |old| old.wrapping_add(6),
                operation: |old| old + 6,
                divisible_by: 19,
                true_throw: 2,
                false_throw: 0,
                inspection_count: 0,
            },
            Monkey {
                nr: 2,
                items: VecDeque::from(vec![79, 60, 97]),
                //operation: |old| old.wrapping_mul(old),
                operation: |old| old * old,
                divisible_by: 13,
                true_throw: 1,
                false_throw: 3,
                inspection_count: 0,
            },
            Monkey {
                nr: 3,
                items: VecDeque::from(vec![74]),
                //operation: |old| old.wrapping_add(3),
                operation: |old| old + 3,
                divisible_by: 17,
                true_throw: 0,
                false_throw: 1,
                inspection_count: 0,
            },
        ];
        MonkeyGroup(monkeys)
    }

    fn create_input() -> MonkeyGroup {
        let monkey = vec![
            Monkey {
                nr: 0,
                items: VecDeque::from(vec![54, 82, 90, 88, 86, 54]),
                operation: |old| old * 7,
                divisible_by: 11,
                true_throw: 2,
                false_throw: 6,
                inspection_count: 0,
            },
            Monkey {
                nr: 1,
                items: VecDeque::from(vec![91, 65]),
                operation: |old| old * 13,
                divisible_by: 5,
                true_throw: 7,
                false_throw: 4,
                inspection_count: 0,
            },
            Monkey {
                nr: 2,
                items: VecDeque::from(vec![62, 54, 57, 92, 83, 63, 63]),
                operation: |old| old + 1,
                divisible_by: 7,
                true_throw: 1,
                false_throw: 7,
                inspection_count: 0,
            },
            Monkey {
                nr: 3,
                items: VecDeque::from(vec![67, 72, 68]),
                operation: |old| old * old,
                divisible_by: 2,
                true_throw: 0,
                false_throw: 6,
                inspection_count: 0,
            },
            Monkey {
                nr: 4,
                items: VecDeque::from(vec![68, 89, 90, 86, 84, 57, 72, 84]),
                operation: |old| old + 7,
                divisible_by: 17,
                true_throw: 3,
                false_throw: 5,
                inspection_count: 0,
            },
            Monkey {
                nr: 5,
                items: VecDeque::from(vec![79, 83, 64, 58]),
                operation: |old| old + 6,
                divisible_by: 13,
                true_throw: 3,
                false_throw: 0,
                inspection_count: 0,
            },
            Monkey {
                nr: 6,
                items: VecDeque::from(vec![96, 72, 89, 70, 88]),
                operation: |old| old + 4,
                divisible_by: 3,
                true_throw: 1,
                false_throw: 2,
                inspection_count: 0,
            },
            Monkey {
                nr: 7,
                items: VecDeque::from(vec![79]),
                operation: |old| old + 8,
                divisible_by: 19,
                true_throw: 4,
                false_throw: 5,
                inspection_count: 0,
            },
        ];
        MonkeyGroup(monkey)
    }
}

impl Debug for MonkeyGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for monkey in &self.0 {
            let item_str = monkey.items.iter().join(", ");
            f.write_str(format!("Monkey {}: {}\n", monkey.nr.to_string(), item_str).as_str())?;
        }
        Ok(())
    }
}
