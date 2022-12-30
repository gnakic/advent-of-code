use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[cfg(test)]
mod day_01_tests {
    use crate::day_01::solution::{
        get_calories_file_contents, most_calories_carried_by_an_elf_sum,
        total_calories_by_top_three_elves,
    };

    #[test]
    fn test_day01_solutions() {
        let calories_file_contents = get_calories_file_contents();

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

pub fn total_calories_by_top_three_elves(calories_file_contents: &str) -> i32 {
    fold_elf_calories::<BinaryHeap<Reverse<i32>>, _>(
        calories_file_contents,
        |top_three_elf_calorie_sums, current_elf_calories_sum| {
            top_three_elf_calorie_sums.push(Reverse(current_elf_calories_sum));

            if top_three_elf_calorie_sums.len() > 3 {
                top_three_elf_calorie_sums.pop();
            }
        },
    )
    .into_iter()
    .fold(0, |acc, calorie_count_result| acc + calorie_count_result.0)
}

pub fn most_calories_carried_by_an_elf_sum(calories_file_contents: &str) -> i32 {
    fold_elf_calories::<i32, _>(
        calories_file_contents,
        |top_elf_calorie_sum, mut current_elf_calories_sum| {
            *top_elf_calorie_sum = *top_elf_calorie_sum.max(&mut current_elf_calories_sum);
        },
    )
}

pub fn fold_elf_calories<T, F>(calories_file_contents: &str, collect: F) -> T
where
    F: Fn(&mut T, i32) -> (),
    T: Default,
{
    let calories_file = calories_file_contents;
    let mut calories_file_lines = calories_file.lines().peekable();

    let mut acc = T::default();

    while calories_file_lines.peek().is_some() {
        let current_elf_calories = calories_file_lines
            .by_ref()
            .take_while(|line| !line.is_empty());

        let total_current_elf_calories =
            current_elf_calories.fold(0, |acc, c| acc + c.parse::<i32>().unwrap());

        collect(&mut acc, total_current_elf_calories);
    }

    acc
}

fn get_calories_file_contents() -> &'static str {
    return include_str!("input/calories.txt");
}
