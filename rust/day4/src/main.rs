use std::{fs::read_to_string, str::FromStr};

fn main() -> Result<(), std::io::Error> {
    let input_file = "input.txt";
    part1(input_file)?;
    part2(input_file)?;

    Ok(())
}

struct Range(usize, usize);

#[derive(Debug)]
struct ParseRangeError;

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseRangeError);
        }

        let parts = s.split("-").collect::<Vec<_>>();

        let start = parts
            .get(0)
            .expect("invalid vec parsed from str")
            .parse::<usize>()
            .unwrap();
        let end = parts
            .get(1)
            .expect("invalid vec parsed from str")
            .parse::<usize>()
            .unwrap();

        Ok(Range(start, end))
    }
}

fn part1(input_file: &str) -> Result<(), std::io::Error> {
    let num_overlaps = get_overlap_count(input_file, |r1, r2| (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1))?;

    println!("Number of partial overlays {}", num_overlaps);

    Ok(())
}


fn part2(input_file: &str) -> Result<(), std::io::Error> {
    let num_overlaps = get_overlap_count(input_file, |r1, r2| (r1.0 <= r2.0 && r1.1 >= r2.0) || (r2.0 <= r1.0 && r2.1 >= r1.0))?;

    println!("Number of partial overlays {}", num_overlaps);

    Ok(())
}

fn get_overlap_count(input_file: &str, is_overlap: fn(Range, Range) -> bool) -> Result<usize, std::io::Error> {
    let num_overlaps = read_to_string(input_file)?
        .lines()
        .filter(|&line| {
            let parts = line.split(",").collect::<Vec<_>>();
            if parts.len() != 2 {
                panic!("failed to parse invalid row \"{}\"", line);
            }

            let r1 = parts
                .get(0)
                .expect("invalid vec len")
                .parse::<Range>()
                .expect("failed to parse range");

            let r2 = parts
                .get(1)
                .expect("invalid vec len")
                .parse::<Range>()
                .expect("failed to parse range");

            is_overlap(r1, r2)
        })
        .count();

    Ok(num_overlaps)
}
