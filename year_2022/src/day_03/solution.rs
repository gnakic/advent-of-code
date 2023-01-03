#[cfg(test)]
mod day_03_tests {
    use crate::day_03::rucksack::Rucksack;

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
}
