use std::collections::BinaryHeap;
use std::iter::Peekable;
use std::str::Lines;

#[cfg(test)]
mod day_01_tests {
    use crate::day_01::solution::{
        most_calories_carried_by_an_elf_sum, total_calories_by_top_three_elves,
    };

    #[test]
    fn test_day01_solutions() {
        let calories_file_contents = include_str!("input/calories.txt");

        assert_eq!(
            most_calories_carried_by_an_elf_sum(calories_file_contents),
            67622
        );

        assert_eq!(
            total_calories_by_top_three_elves(calories_file_contents),
            201491
        );
    }
}

pub fn most_calories_carried_by_an_elf_sum(calories_file_contents: &str) -> i32 {
    ElvesCalories::new(calories_file_contents)
        .into_iter()
        .fold(0, |acc, current_elf_calorie_count| {
            acc.max(current_elf_calorie_count)
        })
}

pub fn total_calories_by_top_three_elves(calories_file_contents: &str) -> i32 {
    ElvesCalories::new(calories_file_contents)
        .into_iter()
        .fold(BinaryHeap::<i32>::new(), |mut acc, current_elf_calories| {
            acc.push(current_elf_calories);
            acc
        })
        .into_iter()
        .take(3)
        .sum::<i32>()
}

struct ElvesCalories<'a>(Peekable<Lines<'a>>);

impl ElvesCalories<'_> {
    fn new(input: &str) -> ElvesCalories<'_> {
        ElvesCalories(input.lines().peekable())
    }
}

impl Iterator for ElvesCalories<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let lines = &mut self.0;

        if lines.peek().is_none() {
            return None;
        }

        let current_elf_calories = lines.by_ref().take_while(|line| !line.is_empty());

        let total_current_elf_calories =
            current_elf_calories.fold(0, |acc, c| acc + c.parse::<i32>().unwrap());

        return Some(total_current_elf_calories);
    }
}
