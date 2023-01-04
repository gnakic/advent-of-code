#[cfg(test)]
mod day_03_tests {
    use std::collections::HashSet;

    use crate::day_03::rucksack::{Rucksack, RucksackItem};

    #[test]
    fn test_day03_solution_part_one() {
        let rucksack_inventory_list = include_str!("input/rucksacks_inventory_list.txt");

        assert_eq!(
            rucksack_inventory_list
                .lines()
                .map(|contents| Rucksack::from(contents))
                .fold(0, |acc, rucksack| {
                    acc + rucksack
                        .unique_item_between_compartments()
                        .map_or(0, |r| r.priority())
                }),
            7872
        );
    }

    #[test]
    fn test_day03_solution_part_two() {
        let mut rucksacks = include_str!("input/rucksacks_inventory_list.txt")
            .lines()
            .map(|contents| Rucksack::from(contents))
            .peekable();

        let mut sum_of_priorities: usize = 0;

        while rucksacks.peek().is_some() {
            let group_rucksacks = rucksacks.by_ref().take(3).collect::<Vec<Rucksack>>();

            let group_rucksacks_item_sets = group_rucksacks
                .iter()
                .by_ref()
                .map(|rucksack| {
                    rucksack
                        .inventory()
                        .into_iter()
                        .collect::<HashSet<&RucksackItem>>()
                })
                .collect::<Vec<HashSet<&RucksackItem>>>();

            let unique_items_between_rucksacks = group_rucksacks_item_sets
                .into_iter()
                .reduce(|acc, inventory_set| acc.intersection(&inventory_set).cloned().collect())
                .unwrap();

            sum_of_priorities += unique_items_between_rucksacks
                .into_iter()
                .fold(0, |acc, item| acc + item.priority());
        }

        assert_eq!(sum_of_priorities, 2497);
    }
}
