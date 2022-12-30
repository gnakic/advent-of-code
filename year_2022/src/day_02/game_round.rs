use std::cmp::Ordering;

pub struct GameRound {
    you: Option<RoundMove>,
    opponent: Option<RoundMove>,
}

#[derive(PartialEq, Eq)]
enum RoundMove {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RoundMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoundMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (RoundMove::Rock, RoundMove::Scissors) => Ordering::Greater,
            (RoundMove::Paper, RoundMove::Rock) => Ordering::Greater,
            (RoundMove::Scissors, RoundMove::Paper) => Ordering::Greater,
            (RoundMove::Rock, RoundMove::Paper) => Ordering::Less,
            (RoundMove::Paper, RoundMove::Scissors) => Ordering::Less,
            (RoundMove::Scissors, RoundMove::Rock) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl GameRound {
    pub fn score(&self) -> i32 {
        self.round_move_score() + self.round_outcome_score()
    }

    fn round_outcome_score(&self) -> i32 {
        match self.you.cmp(&self.opponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }

    fn round_move_score(&self) -> i32 {
        if let Some(your_move) = &self.you {
            match your_move {
                RoundMove::Rock => 1,
                RoundMove::Paper => 2,
                RoundMove::Scissors => 3,
            }
        } else {
            0
        }
    }
}

impl Default for GameRound {
    fn default() -> Self {
        Self {
            you: None,
            opponent: None,
        }
    }
}

impl From<&str> for GameRound {
    fn from(game_round_line: &str) -> Self {
        if let [opponent_move, your_move] = game_round_line.split(" ").collect::<Vec<&str>>()[..] {
            GameRound {
                you: match your_move {
                    "X" => Some(RoundMove::Rock),
                    "Y" => Some(RoundMove::Paper),
                    "Z" => Some(RoundMove::Scissors),
                    _ => None,
                },
                opponent: match opponent_move {
                    "A" => Some(RoundMove::Rock),
                    "B" => Some(RoundMove::Paper),
                    "C" => Some(RoundMove::Scissors),
                    _ => None,
                },
            }
        } else {
            GameRound::default()
        }
    }
}
