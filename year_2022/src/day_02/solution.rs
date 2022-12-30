use super::game_round::{GameRound, PlayerDecryptionStrategy};

#[cfg(test)]
mod day_02_tests {
    use crate::day_02::{game_round::PlayerDecryptionStrategy, solution::total_score};

    #[test]
    fn test_day02_solution_part_one() {
        let game_rounds_contents = include_str!("input/rounds.txt");

        assert_eq!(
            total_score(game_rounds_contents, &PlayerDecryptionStrategy::PartOne),
            14163
        );
    }

    #[test]
    fn test_day02_solution_part_two() {
        let game_rounds_contents = include_str!("input/rounds.txt");

        assert_eq!(
            total_score(game_rounds_contents, &PlayerDecryptionStrategy::PartTwo),
            12091
        );
    }
}

pub fn total_score(game_rounds_contents: &str, strategy: &PlayerDecryptionStrategy) -> i32 {
    game_rounds_contents
        .lines()
        .map(|line| GameRound::from((line, strategy)))
        .fold(0, |acc, round| acc + round.score())
}
