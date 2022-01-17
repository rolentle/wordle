use std::io;
mod words;
use crate::words::*;

fn get_guess() -> String {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("guess: {}", &guess);

    let guess: String = guess.trim().parse().expect("Please type in a word");
    guess
}

fn is_in_dictionary(guess: &str) -> bool {
    VALID_WORDS
        .iter()
        .map(|w| w.to_string())
        .collect::<String>()
        .contains(guess)
}

fn is_five_len(guess: &str) -> bool {
    guess.len() == 5
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
            let guess = get_guess();

            if !is_five_len(&guess) {
                println!(
                    "Guess is must be 5 characters, {} has {} characters. Please guess again.",
                    guess,
                    guess.len()
                );
            } else if !is_in_dictionary(&guess) {
                println!("{} is an invalid word, please guess again.", guess);
            } else if self.answer == guess {
                println!("{} is {}", self.answer, guess);
                println!("You win!");
                break;
            } else {
                println!("{} is incorrect.", guess);
                self.increment_turn();
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
