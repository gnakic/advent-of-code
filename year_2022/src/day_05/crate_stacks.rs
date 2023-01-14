use std::collections::BTreeMap;

use super::crate_move_instruction::CrateMoveInstruction;

pub struct CrateStacks {
    pub arrangement: BTreeMap<usize, Vec<char>>,
    crate_mover: CrateMover,
}

pub enum CrateMover {
    V9000,
    V9001,
}

impl CrateStacks {
    pub fn new(crate_stacks_drawing: Vec<&str>, crate_mover: CrateMover) -> Self {
        let arrangement = Self::parse_crate_arrangement(crate_stacks_drawing);

        CrateStacks {
            arrangement,
            crate_mover,
        }
    }

    fn parse_crate_arrangement(crate_stacks_drawing: Vec<&str>) -> BTreeMap<usize, Vec<char>> {
        let mut crate_stacks_layout = crate_stacks_drawing.into_iter().rev().peekable();

        let number_of_crate_stacks = crate_stacks_layout
            .by_ref()
            .next()
            .unwrap()
            .trim()
            .split("   ")
            .collect::<Vec<&str>>()
            .len();

        let mut crate_stacks_arrangement: BTreeMap<usize, Vec<char>> = BTreeMap::new();

        while crate_stacks_layout.peek().is_some() {
            let current_crate_line = crate_stacks_layout.next();

            if let Some(line) = current_crate_line {
                let mut line = line.chars();

                for crate_stack_index in 1..=number_of_crate_stacks {
                    let current_crate_slot = line
                        .by_ref()
                        .take(4)
                        .filter(|c| c != &' ')
                        .collect::<Vec<char>>();

                    if let ['[', crate_letter, ']'] = current_crate_slot[..] {
                        let current_crate_stack = crate_stacks_arrangement
                            .entry(crate_stack_index)
                            .or_insert(vec![]);

                        current_crate_stack.push(crate_letter);
                    }
                }
            } else {
                break;
            }
        }

        crate_stacks_arrangement
    }

    pub fn move_crates(&mut self, instruction: &CrateMoveInstruction) {
        let from_stack = self.arrangement.get_mut(&instruction.from).unwrap();

        let crates_to_move = from_stack.drain((from_stack.len() - instruction.number_of_crates)..);

        let mut crates_to_move: Vec<char> = match self.crate_mover {
            CrateMover::V9000 => crates_to_move.rev().collect(),
            CrateMover::V9001 => crates_to_move.collect(),
        };

        let to_stack = self.arrangement.get_mut(&instruction.to).unwrap();

        to_stack.append(&mut crates_to_move);
    }

    pub fn top_crates(&self) -> Vec<char> {
        self.arrangement
            .iter()
            .fold(Vec::new(), |mut acc, (_stack_position, stack_crates)| {
                if let Some(top_crate) = stack_crates.last() {
                    acc.push(top_crate.clone());
                }
                acc
            })
    }
}
