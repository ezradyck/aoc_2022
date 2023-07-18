extern crate itertools;

use itertools::Itertools;
use std::fs::read_to_string;

fn main() -> Result<(), std::io::Error> {
    let count = 3;

    let maxes: Vec<u32> = read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .group_by(|x| x.is_empty())
        .into_iter()
        .map(|(_, v)| v.map(|x| x.parse::<u32>().unwrap_or(0)).sum::<u32>())
        .sorted_by(|a, b| b.cmp(a))
        .take(count)
        .collect();

    for (i, max) in maxes.iter().enumerate() {
        println!("{} max value is {}", i + 1, max);
    }

    println!("Sum of maxes is {}", maxes.into_iter().sum::<u32>());

    Ok(())
}
