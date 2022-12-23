#![feature(iter_array_chunks)]

use std::{collections::{HashSet, HashMap}};
use anyhow:: Result;

const START_LOWER: u8 = b'a' - 1;
const START_UPPER: u8 = b'A' - 1;

fn main() -> Result<()> {
    let input = include_str!("./day1_input.txt")
        .lines()
        .array_chunks::<3>()
        .flat_map(|line| {
            return line
                .iter()
                .flat_map(|line| line.chars().collect::<HashSet<_>>().into_iter())
                .fold(HashMap::new(), |mut map: HashMap<char, u32>, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                })
                .into_iter()
                .filter(|(_, v)| *v == 3)
        })
        .map(|c| c.0)
        .map(|c| {
            if c.is_ascii_lowercase() {
                return c as u32 - START_LOWER as u32;
            }

            return c as u32 - START_UPPER as u32 + 26;
        })
        .sum::<u32>();
            // let half = line.len() / 2;
            // // let firstHalf = &line[..half];
            // // let secondHalf = &line[half..];
            // let (a, b) = line.split_at(half); // acheives the same as the above
            // let a = a.chars().collect::<HashSet<_>>();
            // return b
            //     .chars()
            //     .filter(move |c| a.contains(c))
            //     .collect::<HashSet<char>>()
            //     .into_iter()
            //     .map(|c| {
            //         if c.is_ascii_lowercase() {
            //             return c as u8 - START_LOWER;
            //         }

            //         return c as u8 - START_UPPER + 26;
            //     })
            //     .map(|c| c as u32);
        // })
        // .sum::<u32>();
    
    println!("RESULT {:?}", input);

    return Ok(());
}
