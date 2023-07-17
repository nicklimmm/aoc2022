use crate::utils::read_input_lines;

struct Range {
    high: i32,
    low: i32,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let range: Vec<_> = s.split('-').map(|n| n.parse::<i32>().unwrap()).collect();
        assert!(range[0] <= range[1]);
        Range {
            low: range[0],
            high: range[1],
        }
    }

    fn contains(&self, other: &Self) -> bool {
        self.low <= other.low && other.high <= self.high
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.low <= other.high && other.low <= self.high
    }
}

pub fn solve_a() {
    let mut ans = 0;

    for line in read_input_lines("input4.txt") {
        let sections: Vec<_> = line.split(',').collect();

        let first = Range::from_str(sections[0]);
        let second = Range::from_str(sections[1]);

        if first.contains(&second) || second.contains(&first) {
            ans += 1;
        }
    }

    println!("{ans}");
}

pub fn solve_b() {
    let mut ans = 0;

    for line in read_input_lines("input4.txt") {
        let sections: Vec<_> = line.split(',').collect();

        let first = Range::from_str(sections[0]);
        let second = Range::from_str(sections[1]);

        if first.overlaps(&second) {
            ans += 1;
        }
    }

    println!("{ans}");
}
