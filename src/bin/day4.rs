use std::str::FromStr;

use anyhow:: Result;

struct Task {
    start: usize,
    end: usize
}

struct Tasks {
    left: Task,
    right: Task
}

impl Task {
    pub fn contains(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.end || other.start <= self.start && other.end >= self.end;
    }

    pub fn contains_some(&self, other:&Task) -> bool {
        return self.start <= other.start && self.end >= other.start || self.end >= other.end && self.start <= other.end;
    }
}

impl Tasks {
    pub fn contains_another_task(&self) -> bool {
        return self.left.contains(&self.right) || self.right.contains(&self.left);
    }

    pub fn contains_some_other_tasks(&self) -> bool {
        return self.left.contains_some(&self.right) || self.right.contains_some(&self.left);
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a,b) = s.split_once("-").expect("this has to exist");
        return Ok(Task {
            start: a.parse()?,
            end: b.parse()?,
        });
    }
}

impl FromStr for Tasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left,right) = s.split_once(",").expect("this has to exist");
        return Ok(Tasks {
            left: left.parse()?,
            right: right.parse()?
        });
    }
}

fn main() -> Result<()> {
    let input = include_str!("./day4_input.txt")
        .lines()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|task| task.contains_some_other_tasks())
        .count();

    println!("RESULT {:?}", input);
    return Ok(())
}
