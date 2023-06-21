use std::{
    error::Error,
    fmt::Display,
    fs,
    io::{self, Write},
    path::Path,
    str::FromStr,
};

use cli::Config;
use eyre::Result;
use rand::seq::SliceRandom;
pub mod cli;

#[derive(Debug, Clone)]
pub struct Card {
    pub word: String,
    pub translation: String,
}

pub fn run(config: Config) -> Result<()> {
    let mut cards = load_cards(&config.file)?;
    let mut rng = rand::thread_rng();

    let mut round_number = 1;
    while !cards.is_empty() {
        println!();
        println!("======== Round {round_number} ========");
        println!();

        cards.shuffle(&mut rng);
        play_round(&mut cards);
        round_number += 1;
    }

    println!();
    println!("======= Well done! =======");
    println!();

    Ok(())
}

pub fn load_cards(path: &Path) -> Result<Vec<Card>> {
    let csv = fs::read_to_string(path)?;
    let path_utf8 = path.to_string_lossy();
    let cards = csv
        .lines()
        .enumerate()
        .filter_map(|(line, record)| match record.parse::<Card>() {
            Err(err) => {
                eprintln!("warning {path_utf8}:{line}: {err}", line = line + 1);
                None
            }
            Ok(card) => Some(card),
        })
        .collect();
    Ok(cards)
}

impl FromStr for Card {
    type Err = CardFromStrError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut fields = s.split(',');

        let word = fields
            .next()
            .filter(|field| !field.is_empty())
            .ok_or(CardFromStrError::MissingWord)?;

        let translation = fields
            .next()
            .filter(|field| !field.is_empty())
            .ok_or(CardFromStrError::MissingTranslation)?;

        if fields.next().is_some() {
            return Err(CardFromStrError::TooManyFields);
        }

        Ok(Self {
            word: word.to_owned(),
            translation: translation.to_owned(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CardFromStrError {
    MissingWord,
    MissingTranslation,
    TooManyFields,
}

impl Display for CardFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::MissingWord => "missing word",
            Self::MissingTranslation => "missing translation",
            Self::TooManyFields => "too many fields",
        };
        write!(f, "{message}")
    }
}

impl Error for CardFromStrError {}

pub fn play_round(cards: &mut Vec<Card>) {
    *cards = cards
        .drain(..)
        .filter(|card| {
            print!("{word} > ", word = card.word);
            io::stdout().flush().expect("failed to flush stdout");

            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("failed to read answer");

            answer.trim() != card.translation
        })
        .collect();
}
