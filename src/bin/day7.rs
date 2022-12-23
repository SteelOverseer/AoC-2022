use anyhow::{Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./src/bin/day7_prod.txt")?;

    let mut stack = vec![
        ("/", 0)
    ];

    let mut final_count = vec![];
    let total_space = 70000000;
    let space_to_delete = 30000000;
    let reportAmount = 100_000;
    let mut total = 0;

    for line in input.lines().filter(|line| !line.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, amount) = stack.pop().unwrap();
                if amount <= reportAmount {
                    total += amount;
                }
                stack.last_mut().unwrap().1 += amount;
                final_count.push((name, amount))
            } else {
                stack.push((dir, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        } else if amount == "dir" {
            // ignore
        }

    }

    while stack.len() > 0 {
        let (name, amount) = stack.pop().unwrap();
        final_count.push((name, amount));

        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let free_space = total_space - final_count.last().unwrap().1;
    let space_required = space_to_delete - free_space;

    println!("space required = {} = {:?} = {}", space_to_delete, final_count.last().unwrap(), space_required);
    let total = final_count
        .into_iter()
        .filter(move |(_, amount)| *amount >= space_required)
        .map(|(_, amount)| {
            return amount;
        })
        .min();

    println!("total {:?}", total);

    return Ok(());
}