use std::{cmp::Ordering, slice::Iter};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RoundMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl RoundMove {
    pub fn find_move_by_order(value: &Option<RoundMove>, order: Ordering) -> Option<RoundMove> {
        RoundMove::iterator()
            .find(|possible_move| {
                value.as_ref().map_or_else(
                    || false,
                    |opponent_move| possible_move.cmp(&opponent_move) == order,
                )
            })
            .cloned()
    }

    fn iterator() -> Iter<'static, RoundMove> {
        static DIRECTIONS: [RoundMove; 3] =
            [RoundMove::Rock, RoundMove::Paper, RoundMove::Scissors];

        DIRECTIONS.iter()
    }
}

impl PartialOrd for RoundMove {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RoundMove {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return Ordering::Equal;
        }

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
