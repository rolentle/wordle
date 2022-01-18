use std::io;
mod words;
use crate::words::*;
use colored::*;

#[derive(Debug)]
enum MatchType {
    ExtactMatch,
    // PartialMatch,
    NoMatch,
}

#[derive(Debug)]
struct Guess {
    word: String,
    matches: Vec<(char, MatchType)>,
}

impl Guess {
    pub fn new() -> Guess {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("guess: {}", &guess);

        let guess: String = guess.trim().parse().expect("Please type in a word");

        Guess {
            word: guess.clone(),
            matches: guess
                .chars()
                .map(|c| (c, MatchType::NoMatch))
                .collect::<Vec<(char, MatchType)>>(),
        }
    }

    pub fn is_five_len(&self) -> bool {
        self.word.len() == 5
    }

    pub fn is_in_dictionary(&self) -> bool {
        VALID_WORDS
            .iter()
            .map(|w| w.to_string())
            .collect::<String>()
            .contains(&self.word)
    }

    pub fn find_match(&mut self, answer: &str) {
        self.word
            .clone()
            .chars()
            .zip(answer.clone().chars())
            .enumerate()
            .map(|(index, (a_letter, w_letter))| (index, a_letter, a_letter == w_letter))
            .for_each(|(i, a, matched)| {
                if matched {
                    self.matches[i] = (a, MatchType::ExtactMatch);
                }
            });
    }
}

struct Game {
    answer: String,
    turn: u8,
    max_turns: u8,
}

impl Game {
    pub fn new() -> Game {
        Game {
            answer: ANSWERS[0].to_string(),
            turn: 1,
            max_turns: 6,
        }
    }

    pub fn start(&mut self) {
        while self.is_game_over() {
            println!("TURN {}", self.turn);
            let mut guess = Guess::new();

            if self.answer == guess.word {
                println!("{} is {}", self.answer, guess.word);
                println!("You win!");
                break;
            } else if !guess.is_five_len() {
                println!(
                    "Guess is must be 5 characters, {} has {} characters. Please guess again.",
                    guess.word,
                    guess.word.len()
                );
            } else if !guess.is_in_dictionary() {
                println!("{} is an invalid word, please guess again.", guess.word);
            } else {
                println!("{} is incorrect.", &guess.word);
                self.increment_turn();
                guess.find_match(&mut self.answer);
                guess
                    .matches
                    .iter()
                    .for_each(|letter_tuple| println!("{:?}", letter_tuple));
                guess
                    .matches
                    .iter()
                    .for_each(|match_letter| match match_letter.1 {
                        MatchType::ExtactMatch => print!("{}", match_letter.0.to_string().green()),
                        _ => print!("{}", &match_letter.0.to_string().red()),
                    });
                println!("");
            };
        }
        println!("Gameover!");
    }

    fn increment_turn(&mut self) -> u8 {
        self.turn += 1;
        self.turn
    }

    fn is_game_over(&self) -> bool {
        self.turn <= self.max_turns
    }
}

fn main() {
    Game::new().start();
}
