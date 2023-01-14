#[cfg(test)]
mod day_05_tests {

    use crate::day_05::{
        crate_move_instruction::CrateMoveInstruction,
        crate_stacks::{CrateMover, CrateStacks},
    };

    #[test]
    fn test_day05_solution_part_one() {
        let mut cargo_crane_instructions =
            include_str!("input/cargo_crane_instructions.txt").lines();

        let mut crate_stacks = CrateStacks::new(
            cargo_crane_instructions
                .by_ref()
                .take_while(|l| !l.is_empty())
                .collect::<Vec<&str>>(),
            CrateMover::V9000,
        );

        cargo_crane_instructions
            .map(|instruction_line| CrateMoveInstruction::from(instruction_line))
            .for_each(|instruction| {
                crate_stacks.move_crates(&instruction);
            });

        assert_eq!(
            crate_stacks.top_crates().iter().collect::<String>(),
            "QGTHFZBHV"
        );
    }

    #[test]
    fn test_day05_solution_part_two() {
        let mut cargo_crane_instructions =
            include_str!("input/cargo_crane_instructions.txt").lines();

        let mut crate_stacks = CrateStacks::new(
            cargo_crane_instructions
                .by_ref()
                .take_while(|l| !l.is_empty())
                .collect::<Vec<&str>>(),
            CrateMover::V9001,
        );

        cargo_crane_instructions
            .map(|instruction_line| CrateMoveInstruction::from(instruction_line))
            .for_each(|instruction| {
                crate_stacks.move_crates(&instruction);
            });

        assert_eq!(
            crate_stacks.top_crates().iter().collect::<String>(),
            "MGDMPSZTM"
        );
    }
}
