use anyhow::{ Result, Ok};

#[macro_export]
macro_rules! tree_me {
    ($x:ident, $y:ident, $dir_value:ident, $dir_max:ident, $seen:ident) => {
        if $dir_value > $dir_max {
            $seen[$y][$x] += 1;
            $dir_max = $dir_value;
        }
    };
}

macro_rules! tree_me_2 {
    ($x:ident, $y:ident, $height:ident, $trees:ident, $out:ident) => {
        if $trees[$y][$x] < $height {
            $out += 1;
        } else if $trees[$y][$x] == $height {
            $out += 1;
            break;
        } else {
            break;
        }
    };
}

fn see(trees: &Vec<Vec<isize>>, x:usize, y:usize) -> usize {
    let height = trees[y][x];
    let w = trees[0].len();
    let h = trees.len();
    let mut out = 1;
    let mut temp = 0;

    for x in (0..x).rev() {
        tree_me_2!(x, y, height, trees, temp)
    }

    out *= temp;
    temp = 0;
    
    for x in x + 1..w {
        tree_me_2!(x, y, height, trees, temp)
    }
    out *= temp;
    temp = 0;

    for y in (0..y).rev() {
        tree_me_2!(x, y, height, trees, temp)
    }
    out *= temp;
    temp = 0;

    for y in y + 1..h {
        tree_me_2!(x, y, height, trees, temp)
    }
    out *= temp;

    return out;
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./src/bin/day8_prod.txt")?;

    let trees = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            return line
                    .chars()
                    .filter_map(|x| x.to_digit(10))
                    .map(|x| x as isize)
                    .collect::<Vec<isize>>();
        })
        .collect::<Vec<Vec<isize>>>();
        
    let h = trees.len();
    let w = trees[0].len();
    let mut seen = vec![vec![0isize; w]; h];

    // for y in 0..h {
    //     let mut e_H = -1;
    //     let mut w_H = -1;

    //     for x in 0..w {
    //         let w_Idx = w - x - 1;
    //         let east = trees[y][x];
    //         let west = trees[y][w_Idx];

    //         tree_me!(x, y, east, e_H, seen);
    //         tree_me!(w_Idx, y, west, w_H, seen);
    //     }
    // }

    // for x in 0..w {
    //     let mut n_H = -1;
    //     let mut s_H = -1;

    //     for y in 0..h {
    //         let n_Idx = h - y - 1;
    //         let north = trees[n_Idx][x];
    //         let south = trees[y][x];

    //         tree_me!(x, y, south, s_H, seen);
    //         tree_me!(x, n_Idx, north, n_H, seen);
    //     }
    // }

    for y in 0.. h {
        for x in 0..w {
            seen[y][x] = see(&trees, x, y) as isize;
        }
    }
    let count = seen.iter().flat_map(|x| x.iter()).max();

    println!("{:?}", count);
    return Ok(());
}