struct Monkey {
    items: Vec<u128>,
    op: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> usize>,
    inspected: usize,
}

impl Monkey {
    const MOD: u128 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    fn make_turn(&mut self, part_b: bool) -> Vec<(u128, usize)> {
        let result = self
            .items
            .iter()
            .map(|item| {
                let mut level = (self.op)(*item);
                if part_b {
                    level %= Self::MOD;
                } else {
                    level /= 3;
                }

                let to_monkey = (self.test)(level);

                (level, to_monkey)
            })
            .collect();

        self.inspected += self.items.len();
        self.items.clear();

        result
    }
}

fn init_monkeys() -> [Monkey; 8] {
    [
        Monkey {
            items: vec![61],
            op: Box::new(|old| old * 11),
            test: Box::new(|val| if val % 5 == 0 { 7 } else { 4 }),
            inspected: 0,
        },
        Monkey {
            items: vec![76, 92, 53, 93, 79, 86, 81],
            op: Box::new(|old| old + 4),
            test: Box::new(|val| if val % 2 == 0 { 2 } else { 6 }),
            inspected: 0,
        },
        Monkey {
            items: vec![91, 99],
            op: Box::new(|old| old * 19),
            test: Box::new(|val| if val % 13 == 0 { 5 } else { 0 }),
            inspected: 0,
        },
        Monkey {
            items: vec![58, 67, 66],
            op: Box::new(|old| old * old),
            test: Box::new(|val| if val % 7 == 0 { 6 } else { 1 }),
            inspected: 0,
        },
        Monkey {
            items: vec![94, 54, 62, 73],
            op: Box::new(|old| old + 1),
            test: Box::new(|val| if val % 19 == 0 { 3 } else { 7 }),
            inspected: 0,
        },
        Monkey {
            items: vec![59, 95, 51, 58, 58],
            op: Box::new(|old| old + 3),
            test: Box::new(|val| if val % 11 == 0 { 0 } else { 4 }),
            inspected: 0,
        },
        Monkey {
            items: vec![87, 69, 92, 56, 91, 93, 88, 73],
            op: Box::new(|old| old + 8),
            test: Box::new(|val| if val % 3 == 0 { 5 } else { 2 }),
            inspected: 0,
        },
        Monkey {
            items: vec![71, 57, 86, 67, 96, 95],
            op: Box::new(|old| old + 7),
            test: Box::new(|val| if val % 17 == 0 { 3 } else { 1 }),
            inspected: 0,
        },
    ]
}

pub fn solve_a() {
    let mut monkeys = init_monkeys();
    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let result = monkeys[i].make_turn(false);
            for (level, to_monkey) in result {
                monkeys[to_monkey].items.push(level);
            }
        }
    }

    let mut inspected = monkeys.map(|monkey| monkey.inspected);
    inspected.sort();
    inspected.reverse();
    println!("{}", inspected.iter().take(2).product::<usize>());
}

pub fn solve_b() {
    let mut monkeys = init_monkeys();
    let rounds = 10000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let result = monkeys[i].make_turn(true);
            for (level, to_monkey) in result {
                monkeys[to_monkey].items.push(level);
            }
        }
    }

    let mut inspected = monkeys.map(|monkey| monkey.inspected);
    inspected.sort();
    inspected.reverse();
    println!("{}", inspected.iter().take(2).product::<usize>());
}
