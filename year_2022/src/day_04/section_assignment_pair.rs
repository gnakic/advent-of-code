use std::ops::RangeInclusive;

pub struct SectionAssignmentPair(RangeInclusive<i32>, RangeInclusive<i32>);

impl Default for SectionAssignmentPair {
    fn default() -> Self {
        Self(RangeInclusive::new(0, 0), RangeInclusive::new(0, 0))
    }
}

impl From<&str> for SectionAssignmentPair {
    fn from(input: &str) -> Self {
        if let [first_assignment, second_assignment] = input.split(",").collect::<Vec<&str>>()[..] {
            SectionAssignmentPair(
            parse_section_assignment(first_assignment),
            parse_section_assignment(second_assignment),
        )
        } else {
            SectionAssignmentPair::default()
        }
    }
}

fn parse_section_assignment(input: &str) -> RangeInclusive<i32> {
    if let [start, end] = input.split("-").collect::<Vec<&str>>()[..] {
        let start = start.parse::<i32>().unwrap();
        let end = end.parse::<i32>().unwrap();

        start..=end
    } else {
        RangeInclusive::new(0, 0)
    }
}

impl SectionAssignmentPair {
    pub fn has_fully_overlapping_assignments(&self) -> bool {
        let SectionAssignmentPair(first_assignment, second_assignment) = &self;

        (first_assignment.start() >= second_assignment.start()
            && first_assignment.end() <= second_assignment.end())
            || (second_assignment.start() >= first_assignment.start()
                && second_assignment.end() <= first_assignment.end())
    }
}
