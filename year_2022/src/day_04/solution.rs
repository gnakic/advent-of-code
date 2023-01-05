#[cfg(test)]
mod day_04_tests {
    use crate::day_04::section_assignment_pair::SectionAssignmentPair;

    #[test]
    fn test_day04_solution_part_one() {
        let section_assignment_pairs_input = include_str!("input/section_assignment_pairs.txt");

        assert_eq!(
            section_assignment_pairs_input
                .lines()
                .map(|assignment_pair_input| SectionAssignmentPair::from(assignment_pair_input))
                .filter(|pair| pair.has_fully_duplicated_assignments())
                .count(),
            511
        );
    }

    #[test]
    fn test_day04_solution_part_two() {
        let section_assignment_pairs_input = include_str!("input/section_assignment_pairs.txt");

        assert_eq!(
            section_assignment_pairs_input
                .lines()
                .map(|assignment_pair_input| SectionAssignmentPair::from(assignment_pair_input))
                .filter(|pair| pair.has_overlapping_assignments())
                .count(),
            821
        );
    }
}
