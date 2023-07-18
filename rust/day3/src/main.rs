extern crate bit_set;

use bit_set::BitSet;
use std::fs::read_to_string;

fn main() -> Result<(), std::io::Error> {
    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> Result<(), std::io::Error> {
    let total_priority: usize = read_to_string("input.txt")?
        .lines()
        .fold(0, |priority, line| {
            let first_compartment = get_bitmap(line, 0, line.len() / 2);
            let second_compartment = get_bitmap(line, line.len() / 2, line.len() / 2);

            let line_priority: usize = first_compartment
                .intersection(&second_compartment)
                .into_iter()
                .map(|x| x + 1)
                .sum();

            priority + line_priority
        });

    println!("total priority {}", total_priority);

    Ok(())
}

fn part2() -> Result<(), std::io::Error> {
    let total_priority: usize = read_to_string("input.txt")?
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|v| {
            let mut group_map = BitSet::new();
            let mut is_first = true;

            for map in v.iter().map(|line| get_bitmap(line, 0, line.len())) {
                if is_first {
                    group_map = map;
                    is_first = false;
                } else {
                    group_map.intersect_with(&map);
                }
            }

            return group_map.into_iter().map(|x| x + 1).sum::<usize>();
        })
        .sum();

    println!("sum of priorities for groups {}", total_priority);

    Ok(())
}

fn get_bitmap(line: &str, offset: usize, len: usize) -> BitSet {
    line.as_bytes()
        .into_iter()
        .skip(offset)
        .take(len)
        .fold(BitSet::new(), |mut acc, &x| {
            let index = match x {
                b'A'..=b'Z' => {
                    let letter = x - b'A';
                    Some(letter + 26)
                }
                b'a'..=b'z' => {
                    let letter = x - b'a';
                    Some(letter)
                }
                _ => None,
            };

            if let Some(index) = index {
                acc.insert(index.into());
            }

            acc
        })
}
