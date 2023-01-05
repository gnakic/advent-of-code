#[derive(Default)]
pub struct SectionAssignmentPair(SectionAssignment, SectionAssignment);

impl From<&str> for SectionAssignmentPair {
    fn from(input: &str) -> Self {
        if let [first_assignment, second_assignment] = input.split(",").collect::<Vec<&str>>()[..] {
            SectionAssignmentPair(
                SectionAssignment::from(first_assignment),
                SectionAssignment::from(second_assignment),
            )
        } else {
            SectionAssignmentPair::default()
        }
    }
}

impl SectionAssignmentPair {
    pub fn has_fully_duplicated_assignments(&self) -> bool {
        let SectionAssignmentPair(first_assignment, second_assignment) = &self;

        first_assignment.contains(second_assignment) || second_assignment.contains(first_assignment)
    }

    pub fn has_overlapping_assignments(&self) -> bool {
        let SectionAssignmentPair(first_assignment, second_assignment) = &self;

        first_assignment.overlaps(second_assignment)
    }
}

#[derive(Default)]
pub struct SectionAssignment {
    from: i32,
    to: i32,
}

impl From<&str> for SectionAssignment {
    fn from(input: &str) -> Self {
        if let [from, to] = input.split("-").collect::<Vec<&str>>()[..] {
            let from = from.parse::<i32>().unwrap();
            let to = to.parse::<i32>().unwrap();

            Self { from, to }
        } else {
            Self::default()
        }
    }
}

impl SectionAssignment {
    fn overlaps(&self, other: &Self) -> bool {
        self.from <= other.to && self.to >= other.from
    }

    fn contains(&self, other: &Self) -> bool {
        other.from >= self.from && other.to <= self.to
    }
}
