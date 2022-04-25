use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::{stdin, stdout, Write};
use std::{fs, time::Instant};

const GREEN: &str = "\x1b[30m\x1b[42m";
const YELLOW: &str = "\x1b[30m\x1b[43m";
const RESET: &str = "\x1b[0m";
const CLEAR: &str = "\x1b[H\x1b[2J";

#[derive(Serialize, Deserialize)]
struct WordList {
    words: Vec<String>,
}

macro_rules! load_data {
    ($path:expr) => {
        match fs::read_to_string($path) {
            Ok(v) => {
                let data: Vec<String> = serde_json::from_str(&v)?;
                data
            }
            Err(_) => panic!("failed to read from file!"),
        }
    };
}

struct Board {
    board: Vec<Vec<String>>,
    target_word: String,
}

impl Board {
    pub fn new(length: usize, word: String) -> Self {
        Board {
            board: vec![vec![String::from(""); length]; length],
            target_word: word,
        }
    }
    pub fn add_guess(&mut self, guess: String) -> bool {
        let characters = guess.trim().split("").collect::<Vec<&str>>();
        for r in self.board.iter_mut() {
            if r[0].trim().len() != 0 {
                continue;
            }
            for (i, c) in characters.iter().enumerate() {
                if c == &"" {
                    continue;
                }
                r[i - 1] = c.to_string();
            }
            return guess.trim() == self.target_word.trim();
        }
        return false;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars: Vec<&str> = self.target_word.split("").collect();
        chars.retain(|x| x.len() == 1);
        for r in &self.board {
            for (i, c) in r.iter().enumerate() {
                let mut color = "";
                if c == &chars[i] {
                    color = GREEN;
                } else if chars.contains(&c.as_str()) {
                    color = YELLOW;
                }
                write!(f, "{color} {c} {RESET}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

macro_rules! load_file {
    ($val:expr) => {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open($val)?
    };
}

type BResult<T> = Result<T, Box<dyn Error>>;

fn update_list(target_word: &str) -> BResult<()> {
    let mut list: Vec<String> = load_data!("./words.json");
    list.retain(|x| x != target_word);
    let mut unused_file = load_file!("./words.json");
    let list = serde_json::to_string(&list).unwrap();
    unused_file.set_len(list.as_bytes().len() as u64)?;
    unused_file.write_all(list.as_bytes())?;
    let mut used = load_data!("./used.json");
    let mut used_file = load_file!("./used.json");
    used.append(&mut vec![target_word.to_string()]);
    used_file.write_all(serde_json::to_string(&used).unwrap().as_bytes())?;
    Ok(())
}

fn main() -> BResult<()> {
    let start = Instant::now();
    let args = env::args().skip(1).collect::<Vec<_>>();
    let word_length = match args.len() {
        1.. => args[0].parse::<usize>().unwrap_or(5),
        _ => 5,
    };
    let mut word_list: Vec<String> = load_data!("./words.json");
    word_list.retain(|x| x.len() == word_length);
    if word_list.len() == 0 {
        eprintln!("no words of this length!");
        std::process::exit(0);
    }
    let selected_word = &word_list[thread_rng().gen_range(0..word_list.len())].to_lowercase();
    println!("{CLEAR}Word Length: {word_length}  guesses: {word_length}");
    let mut board = Board::new(word_length, selected_word.trim().to_string());
    for _ in 1..=word_length {
        loop {
            let mut input = String::with_capacity(word_length);
            stdout().flush()?;
            stdin().read_line(&mut input)?;
            if input.trim().len() == word_length {
                if board.add_guess(input) {
                    println!("{CLEAR}{board}\nelapsed: {:#?}", start.elapsed());
                    update_list(&selected_word.trim())?;
                    std::process::exit(0);
                };
                println!("{CLEAR}{board}\nelapsed: {:#?}", start.elapsed());
                break;
            }
            println!("invalid length");
        }
    }
    println!("better luck next time (:\nthe word was {selected_word}");
    Ok(())
}
