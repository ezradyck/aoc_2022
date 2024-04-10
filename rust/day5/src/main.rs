use regex::Regex;
use std::{fmt::Display, fs::read_to_string};

fn main() -> Result<(), std::io::Error> {
    let input_filename = "input.txt";

    let binding = read_to_string(input_filename)?;
    let mut crates = Vec::new();
    let mut instructions = Vec::new();
    let mut crates_read = false;

    for line in binding.lines() {
        if line.is_empty() {
            crates_read = true;
        } else if !crates_read {
            crates.push(line.clone());
        } else {
            instructions.push(line);
        }
    }

    let placements: Vec<_> = crates
        .last()
        .expect("unable to get last crate line")
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u8>().expect("unable to parse placement"))
        .collect();

    let mut placements: Vec<Placement> = placements
        .iter()
        .map(|p| {
            let mut cs = Vec::new();
            for c in crates.iter().rev().skip(1) {
                let index = 1 + ((p - 1) * 4);
                let c = c.chars().nth(index.into()).unwrap();
                if c.is_alphabetic() {
                    cs.push(Crate { 0: c.to_string() });
                }
            }

            Placement {
                id: p.clone(),
                crates: cs,
            }
        })
        .collect();

    let r = Regex::new(r"move (?<num>\d*) from (?<from>\d*) to (?<to>\d*)").unwrap();

    for i in instructions {
        let captures = r.captures(i).expect("unable to capture regex on line");
        let from = &captures["from"].parse::<u8>().unwrap();
        let to = &captures["to"].parse::<u8>().unwrap();
        let num = &captures["num"].parse::<u8>().unwrap();

        let from: &mut Placement = placements.iter_mut().find(|p| p.id == *from).unwrap();

        let mut moved = Vec::new();
        for _ in 0..*num {
            moved.push(from.crates.pop().unwrap());
        }
        moved.reverse();
        println!("moving: {:?} for {}", moved, i);

        let to: &mut Placement = placements.iter_mut().find(|p| p.id == *to).unwrap();
        to.crates.append(&mut moved.clone());
    }

    for p in &placements {
        println!("Placement {}: {}", p.id, p.crates.last().unwrap_or(&Crate { 0: String::new() }));
    }

    let message = placements
        .iter()
        .map(|p| p.crates.last().unwrap_or(&Crate { 0: String::new() }).0.clone())
        .fold(String::new(), |acc, c| acc + &c);

    println!("Message: {}", message);

    Ok(())
}

#[derive(Debug)]
struct Crate(String);

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str(&self.0);
        Ok(())
    }
}

impl Clone for Crate {
    fn clone(&self) -> Self {
        Crate { 0: self.0.clone() }
    }
}

#[derive(Debug)]
struct Placement {
    id: u8,
    crates: Vec<Crate>,
}
