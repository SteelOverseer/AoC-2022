use std::str::FromStr;
use anyhow::{Result, Ok};

struct Movement {
    direction: char,
    steps: usize
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let(a, b) = s.split_once(" ").expect("should be there");
        return Ok(Movement { direction: a.parse()?, steps: b.parse()?});
    }
}

fn main() -> Result<()> {
    let position = (0,0);

    let input = std::fs::read_to_string("./src/bin/day8_prod.txt")?;

    let result = input
        .lines()
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse::<Movement>());

    return Ok(());
}