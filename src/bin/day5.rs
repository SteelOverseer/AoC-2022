use std::fs;
use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Stack {
    crates: Vec<Vec<Option<char>>>
}

impl Stack {
    fn move_crate(&mut self, m: &Operation) {
        for _ in 0..m.count {
            let c = self.crates[m.from].pop().expect("should exist");
            self.crates[m.to].push(c);
        }
    }

    fn move_multiple_crates(&mut self, m: &Operation) {
        let out_items = (0..m.count)
            .filter_map(|_| self.crates[m.from].pop())
            .collect::<Vec<_>>();
        for item in out_items.into_iter().rev() {
            self.crates[m.to].push(item);
        }
    }

    fn print_top(&self) {
        print!("Stack: ");
        for stack in &self.crates {
            print!("{}", stack.last().expect("test").expect("again"));
        }
        println!("")
    }
}

struct Operation {
    count: usize,
    from: usize,
    to: usize
}

impl Operation {
    fn new(count: usize, from: usize, to: usize) -> Self {
        Operation {
            count,
            from,
            to
        }
    }
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return Ok(s
            .split_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .collect::<Operation>());
    }
}

impl FromIterator<usize> for Operation {

    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        return Operation {
            count: iter.next().expect("must exist"),
            from: iter.next().expect("must exist") - 1,
            to: iter.next().expect("must exist") - 1
        };
    }
}

impl FromStr for Stack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut crates = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut idx = 0;
            let mut stack_idx = 0;

            while idx < line.len() {
                if crates.len() <= stack_idx {
                    crates.push(Vec::new());
                }

                if line[idx..].starts_with("[") {
                    let c = line.chars().nth(idx + 1).expect("must exist");
                    crates[stack_idx].push(Some(c))
                }

                idx += 4;
                stack_idx += 1;
            }
        }

        return Ok(Stack {
            crates
        });
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("./src/bin/day5_prod.txt")?;
    let (stack, moves) = file
        .split_once("\n\n")
        .expect("should not break");
    
    let mut stack = stack.parse::<Stack>()?;
    let moves = moves
        .lines()
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse::<Operation>())
        .collect::<Vec<Operation>>();

    for m in moves {
        stack.move_multiple_crates(&m)
    }

    stack.print_top();
 
    return Ok(());
}