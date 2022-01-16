use std::io;
mod words;
use crate::words::*;

fn main() {
    let answer = ANSWERS[0];
        println!("Please input your guess.");

    let mut turn: u8 = 1;
    
    while turn < 7 {
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        println!("guess: {}", &guess);

        let guess: String = guess.trim().parse().expect("Please type in a word");
        let is_valid_word: bool = VALID_WORDS
            .iter()
            .map(|w| w.to_string())
            .collect::<String>()
            .contains(&guess);
        if is_valid_word {
            if answer == guess {
                println!("{} is {}",answer, guess);
                break
            } else {
                turn += 1;
                println!("{} is incorrect", guess);
            }
        } else {
            turn += 1;
            println!("{} is invalid", guess);
        };
    }
    println!("Gameover!");
}
