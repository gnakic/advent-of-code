use super::round_move::RoundMove;
use std::cmp::Ordering;

pub struct GameRound {
    player: Option<RoundMove>,
    opponent: Option<RoundMove>,
}

enum GameRoundScore {
    Draw = 3,
    Won = 6,
    Lost = 0,
}

impl GameRound {
    pub fn score(&self) -> i32 {
        self.player.map_or_else(
            || 0,
            |player_move| player_move as i32 + self.round_outcome_score() as i32,
        )
    }

    fn round_outcome_score(&self) -> GameRoundScore {
        match self.player.cmp(&self.opponent) {
            Ordering::Less => GameRoundScore::Lost,
            Ordering::Equal => GameRoundScore::Draw,
            Ordering::Greater => GameRoundScore::Won,
        }
    }
}

impl Default for GameRound {
    fn default() -> Self {
        Self {
            player: None,
            opponent: None,
        }
    }
}

#[derive(PartialEq)]
pub enum PlayerDecryptionStrategy {
    PartOne,
    PartTwo,
}

impl From<(&str, &PlayerDecryptionStrategy)> for GameRound {
    fn from(input: (&str, &PlayerDecryptionStrategy)) -> Self {
        let (game_round_line, strategy) = input;

        if let [encrypted_opponent_move, encrypted_player_move] =
            game_round_line.split(" ").collect::<Vec<&str>>()[..]
        {
            let opponent_round_move = match encrypted_opponent_move {
                "A" => Some(RoundMove::Rock),
                "B" => Some(RoundMove::Paper),
                "C" => Some(RoundMove::Scissors),
                _ => None,
            };

            let player_round_move = if strategy == &PlayerDecryptionStrategy::PartOne {
                match encrypted_player_move {
                    "X" => Some(RoundMove::Rock),
                    "Y" => Some(RoundMove::Paper),
                    "Z" => Some(RoundMove::Scissors),
                    _ => None,
                }
            } else {
                match encrypted_player_move {
                    "X" => RoundMove::find_move_by_order(&opponent_round_move, Ordering::Less),
                    "Y" => opponent_round_move.clone(),
                    "Z" => RoundMove::find_move_by_order(&opponent_round_move, Ordering::Greater),
                    _ => None,
                }
            };

            GameRound {
                player: player_round_move,
                opponent: opponent_round_move,
            }
        } else {
            GameRound::default()
        }
    }
}
