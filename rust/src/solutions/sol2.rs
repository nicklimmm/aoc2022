use crate::utils::read_input_lines;

enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn points(&self) -> i32 {
        match *self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid"),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn from_str(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Invalid"),
        }
    }

    fn from_outcome(other: &Self, outcome: &Outcome) -> Self {
        match outcome {
            Outcome::Draw => other.clone(),
            Outcome::Win => match other {
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
                Self::Rock => Self::Paper,
            },
            Outcome::Lose => match other {
                Self::Rock => Self::Scissors,
                Self::Scissors => Self::Paper,
                Self::Paper => Self::Rock,
            },
        }
    }

    fn against(&self, other: &Self) -> i32 {
        self.outcome_against(other).points() + self.points()
    }

    fn outcome_against(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (Action::Paper, Action::Rock)
            | (Action::Rock, Action::Scissors)
            | (Action::Scissors, Action::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }

    fn points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub fn solve_a() {
    let mut total_score = 0;

    for line in read_input_lines("input2.txt") {
        let actions: Vec<&str> = line.split_whitespace().collect();
        let (opp_action, my_action) = (Action::from_str(actions[0]), Action::from_str(actions[1]));
        total_score += my_action.against(&opp_action);
    }

    println!("{total_score}");
}

pub fn solve_b() {
    let mut total_score = 0;

    for line in read_input_lines("input2.txt") {
        let inputs: Vec<&str> = line.split_whitespace().collect();
        let (opp_action, outcome) = (Action::from_str(inputs[0]), Outcome::from_str(inputs[1]));
        total_score += Action::from_outcome(&opp_action, &outcome).points() + outcome.points();
    }

    println!("{total_score}");
}
