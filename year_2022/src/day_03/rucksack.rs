use once_cell::sync::Lazy;
use std::collections::HashSet;

static ITEM_PRIORITIES: Lazy<Vec<char>> =
    Lazy::new(|| ('a'..='z').chain('A'..='Z').collect::<Vec<char>>());

pub struct Rucksack {
    first_compartment: Vec<RucksackItem>,
    second_compartment: Vec<RucksackItem>,
}

impl<'a> From<&'a str> for Rucksack {
    fn from(rucksack_contents: &'a str) -> Self {
        let rucksack_length = rucksack_contents.len();
        let (first_compartment, second_compartment) =
            rucksack_contents.split_at(rucksack_length / 2);

        Rucksack {
            first_compartment: first_compartment
                .chars()
                .map(|item| RucksackItem::from(item))
                .collect(),
            second_compartment: second_compartment
                .chars()
                .map(|item| RucksackItem::from(item))
                .collect(),
        }
    }
}

impl Rucksack {
    pub fn unique_item_between_compartments(&self) -> Option<&RucksackItem> {
        let first_compartment: HashSet<&RucksackItem> = self.first_compartment.iter().collect();
        let second_compartment: HashSet<&RucksackItem> = self.second_compartment.iter().collect();

        let intersection_result: Vec<&&RucksackItem> = first_compartment
            .intersection(&second_compartment)
            .collect();

        if intersection_result.len() == 1 {
            Some(intersection_result[0])
        } else {
            None
        }
    }

    pub fn inventory(&self) -> Vec<&RucksackItem> {
        self.first_compartment
            .iter()
            .chain(self.second_compartment.iter())
            .collect()
    }
}

#[derive(Eq, Hash)]
pub struct RucksackItem(char);

impl From<char> for RucksackItem {
    fn from(item: char) -> Self {
        RucksackItem(item)
    }
}

impl PartialEq for RucksackItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
    }
}

impl RucksackItem {
    pub fn priority(&self) -> usize {
        ITEM_PRIORITIES.iter().position(|&x| x == self.0).unwrap() + 1
    }
}
