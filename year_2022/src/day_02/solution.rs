use super::game_round::GameRound;

#[cfg(test)]
mod day_02_tests {
    use crate::day_02::solution::total_score;

    #[test]
    fn test_day02_solutions() {
        let game_rounds_contents = include_str!("input/rounds.txt");

        assert_eq!(total_score(game_rounds_contents), 12091);
    }
}

pub fn total_score(game_rounds_contents: &str) -> i32 {
    game_rounds_contents
        .lines()
        .map(|line| GameRound::from(line))
        .fold(0, |acc, round| acc + round.score())
}
