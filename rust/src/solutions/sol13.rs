use std::cmp::Ordering;

#[derive(Clone)]
enum DistressSignal {
    List(Vec<Self>),
    Int(i64),
}

impl From<&str> for DistressSignal {
    fn from(s: &str) -> Self {
        let json = serde_json::from_str::<serde_json::Value>(s).unwrap();
        Self::from(json)
    }
}

impl From<serde_json::Value> for DistressSignal {
    fn from(json: serde_json::Value) -> Self {
        match json {
            serde_json::Value::Array(arr) => Self::List(arr.into_iter().map(Self::from).collect()),
            serde_json::Value::Number(n) => Self::Int(n.as_i64().unwrap()),
            _ => panic!(),
        }
    }
}

impl PartialEq for DistressSignal {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Equal))
    }
}

impl Eq for DistressSignal {}

impl PartialOrd for DistressSignal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Some(a.cmp(b)),
            (Self::List(a), Self::List(b)) => {
                let mut i = 0;

                while i < a.len() && i < b.len() {
                    match a[i].partial_cmp(&b[i]).unwrap() {
                        Ordering::Equal => i += 1,
                        ord => return Some(ord),
                    }
                }

                Some((a.len()).cmp(&b.len()))
            }
            (Self::List(_), Self::Int(_)) => {
                let b = &Self::List(vec![other.clone()]);
                self.partial_cmp(b)
            }
            (Self::Int(_), Self::List(_)) => {
                let a = &Self::List(vec![self.clone()]);
                a.partial_cmp(other)
            }
        }
    }
}

impl Ord for DistressSignal {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn solve_a() {
    let input = include_str!("inputs/input13.txt")
        .trim()
        .split("\n\n")
        .flat_map(|pair| pair.split('\n').map(DistressSignal::from))
        .collect::<Vec<DistressSignal>>();

    let mut ans = 0;
    for (i, pair) in input.chunks_exact(2).enumerate() {
        if pair[0] <= pair[1] {
            ans += i + 1;
        }
    }

    println!("{ans}");
}

pub fn solve_b() {
    let mut input = include_str!("inputs/input13.txt")
        .trim()
        .split("\n\n")
        .flat_map(|pair| pair.split('\n').map(DistressSignal::from))
        .collect::<Vec<DistressSignal>>();

    // To sequentially check valid divider insertions
    input.sort();

    let mut first_divider_done = false;
    let first_divider = DistressSignal::from("[[2]]");
    let second_divider = DistressSignal::from("[[6]]");

    let mut ans = 1;
    for (i, pair) in input.windows(2).enumerate() {
        if !first_divider_done && pair[0] <= first_divider && first_divider <= pair[1] {
            // Compensate 0-indexing, window size
            ans *= i + 2;
            first_divider_done = true;
        } else if first_divider_done && pair[0] <= second_divider && second_divider <= pair[1] {
            // Compensate 0-indexing, window size, placement of first divider
            ans *= i + 3;
            break;
        }
    }
    println!("{ans}");
}
