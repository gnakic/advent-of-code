use once_cell::sync::Lazy;
use regex::Regex;

static INSTRUCTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"move (?P<number_of_crates>\d+) from (?P<from>\d+) to (?P<to>\d+)")
        .unwrap()
});

pub struct CrateMoveInstruction {
    pub number_of_crates: usize,
    pub from: usize,
    pub to: usize,
}

impl From<&str> for CrateMoveInstruction {
    fn from(raw_instruction: &str) -> Self {
        let captures = INSTRUCTION_REGEX.captures(raw_instruction).unwrap();

        CrateMoveInstruction {
            number_of_crates: captures["number_of_crates"].parse::<usize>().unwrap(),
            from: captures["from"].parse::<usize>().unwrap(),
            to: captures["to"].parse::<usize>().unwrap(),
        }
    }
}
