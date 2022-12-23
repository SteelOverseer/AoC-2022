use std::str::FromStr;

use anyhow::{ Result, Ok};

/*
**** SCORING ****

1 - Rock
2 - Paper
3 - Scissors

0 - Loss
3 - Draw
6 - Win

**** KEY ****

A/X - Rock
B/Y - Paper
C/Z - Scissors
*/

#[derive(Debug)]
struct HandPair_1 {
    value: usize
}

struct HandPair_2 {
    value: usize
}

// 2 + A(0)
// 2 + B(1)
// 2 + C(2)
const WIN_LOSE: [usize; 3] = [3, 6, 0];
const WIN_LOSE_2: [usize; 3] = [0, 3, 6];
const CHOICE_VALUE: [usize; 3] = [3, 1, 2];

fn to_number(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 2,
        "C" => 1,
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!("oopsie")
    };
}

fn to_number_2(c: &str) -> usize {
    return match c {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => unreachable!("oopsie")
    };
}


impl FromStr for HandPair_1 {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        let(o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input"))
        };

        let o2 = to_number(o);
        let p2 = to_number(p);
        let score = p2 + WIN_LOSE[(2 + o2 + p2) % WIN_LOSE.len()];

        return Ok(HandPair_1 { value: score });
    }
}

impl FromStr for HandPair_2 {
    type Err = anyhow::Error;
    
    fn from_str(s: &str) -> Result<Self> {
        let(o, p) = match s.split_once(" ") {
            Some((o, p)) => (o, p),
            None => return Err(anyhow::anyhow!("invalid input"))
        };

        let o2 = to_number_2(o);
        let p2 = to_number_2(p);
        let score = CHOICE_VALUE[(o2 + p2) % CHOICE_VALUE.len()] + WIN_LOSE_2[p2];

        return Ok(HandPair_2 { value: score });
    }
}

fn main() -> Result<()> {
    let values:usize = include_str!("./day2_input.txt")
        .lines()
        .flat_map(|x| x.parse::<HandPair_2>())
        .map(|x| x.value)
        .sum();
    
    println!("Values: {:?}", values);

    return Ok(());
}

// MY ATTEMPT BELOW

// fn main() -> Result<()> {
//     let mut matches = include_str!("./input.txt").split("\n\n").collect::<Vec<&str>>();
//     // let mut rounds = matches();
    
//     println!("What is this?? {:?}", matches);
//     // max.sort_by(|a,b| b.cmp(a));

//     // println!("MAX {:?}", max
//     //     .iter()
//     //     .take(3)
//     //     .sum::<usize>()
//     // );

//     return Ok(());
// }

// fn get_round_score(opp_val: &str, my_val: &str) -> u32 {
//     let mut score = 0;

//     if (opp_val == "A" && my_val == "X") || (opp_val == "B" && my_val == "Y") || (opp_val == "C" && my_val == "Z") {
//         // Draw
//         score += 3;
//     } else if (opp_val == "A" && my_val == "Y") || (opp_val == "B" && my_val == "Z") || (opp_val == "C" && my_val == "X") {
//         // Win
//         score += 6;
//     } else {
//         // Loss
//         score += 0;
//     }

//     if my_val == "X" {
//         score += 1;
//     } else if my_val == "Y" {
//         score += 2;
//     } else {
//         score += 3;
//     }

//     return score;
// }