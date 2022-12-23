use anyhow:: Result;

fn main() -> Result<()> {
    let mut max = include_str!("../input.txt")
        .split("\n\n")
        .map(|x| {
            return x
                .lines()
                .flat_map(str::parse::<usize>)
                .sum::<usize>();
        })
        .collect::<Vec<usize>>();
    
    max.sort_by(|a,b| b.cmp(a));

    println!("MAX {:?}", max
        .iter()
        .take(3)
        .sum::<usize>()
    );
    return Ok(());
}
