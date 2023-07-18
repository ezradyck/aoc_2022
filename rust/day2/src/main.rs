use std::fs::read_to_string;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Outcome {
    Win,
    Lose,
    Tie,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOutcomeError;

impl FromStr for Outcome {
    type Err = ParseOutcomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Tie),
            "Z" => Ok(Outcome::Win),
            _ => Err(ParseOutcomeError {}),
        }
    }
}

impl From<&char> for Outcome {
    fn from(value: &char) -> Self {
        value
            .to_string()
            .parse::<Outcome>()
            .expect("could not parse char into outcome")
    }
}

impl Outcome {
    fn get_required_selection(&self, their_selection: &Selection) -> Selection {
        match self {
            Outcome::Tie => their_selection.clone(),
            Outcome::Lose => their_selection.get_losing_selection(),
            Outcome::Win => their_selection.get_winning_selection(),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSelectionError;

impl FromStr for Selection {
    type Err = ParseSelectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Selection::Rock),
            "B" => Ok(Selection::Paper),
            "C" => Ok(Selection::Scissors),
            _ => Err(ParseSelectionError {}),
        }
    }
}

impl From<&char> for Selection {
    fn from(value: &char) -> Self {
        value
            .to_string()
            .parse::<Selection>()
            .expect("could not parse char into selection")
    }
}

impl Selection {
    fn get_round_outcome(&self, theirs: &Self) -> Outcome {
        if self.eq(theirs) {
            return Outcome::Tie;
        }

        match (self, theirs) {
            (Selection::Rock, Selection::Scissors)
            | (Selection::Paper, Selection::Rock)
            | (Selection::Scissors, Selection::Paper) => Outcome::Win,
            _ => Outcome::Lose,
        }
    }

    fn get_losing_selection(&self) -> Selection {
        match self {
            Selection::Rock => Selection::Scissors,
            Selection::Scissors => Selection::Paper,
            Selection::Paper => Selection::Rock,
        }
    }

    fn get_winning_selection(&self) -> Selection {
        match self {
            Selection::Scissors => Selection::Rock,
            Selection::Paper => Selection::Scissors,
            Selection::Rock => Selection::Paper,
        }
    }
}

fn calculate_round_score(theirs: Selection, ours: Selection) -> u32 {
    let selection_score = match ours {
        Selection::Rock => 1,
        Selection::Paper => 2,
        Selection::Scissors => 3,
    };

    let outcome_score = match ours.get_round_outcome(&theirs) {
        Outcome::Win => 6,
        Outcome::Tie => 3,
        Outcome::Lose => 0,
    };

    return selection_score + outcome_score;
}

fn main() -> Result<(), std::io::Error> {
    let total_score = read_to_string("input.txt")?
        .lines()
        .map(|line| {
            let chars = Vec::from_iter(line.chars());
            let theirs = Selection::from(
                chars
                    .get(0)
                    .expect("character not found for their selection"),
            );
            let required_outcome = Outcome::from(
                chars 
                    .get(2)
                    .expect("character not found for required outcome"),
            );
            let ours = required_outcome.get_required_selection(&theirs);

            return calculate_round_score(theirs, ours);
        })
        .sum::<u32>();

    println!("total score is {}", total_score);

    Ok(())
}
